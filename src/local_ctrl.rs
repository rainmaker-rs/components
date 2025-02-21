use quick_protobuf::{MessageWrite, Writer};

use crate::{
    http::HttpConfiguration,
    proto::local_ctrl::{LocalCtrlMessage, LocalCtrlMsgType},
    protocomm::ProtocommHttpd,
    utils::{wrap_in_arc_mutex, WrappedInArcMutex},
};

pub type PropGetValCb = Box<dyn Fn(&str, u32, u32) -> Vec<u8> + Send>;
pub type PropSetValCb = Box<dyn Fn(&str, u32, u32, Vec<u8>) + Send>;

struct LocalControlSharedData {
    properties: Vec<Property>,
    get_prop_vals_cb: PropGetValCb,
    set_prop_vals_cb: PropSetValCb,
}

struct Property {
    pub name: String,
    pub property_type: u32,
    pub flags: u32,
}

pub struct LocalControl {
    #[allow(dead_code)]
    protocomm: ProtocommHttpd,
    shared: WrappedInArcMutex<LocalControlSharedData>,
}

impl LocalControl {
    pub fn new(get_prop_vals_cb: PropGetValCb, set_prop_vals_cb: PropSetValCb) -> Self {
        let mut protocomm = ProtocommHttpd::new(
            HttpConfiguration {
                port: 8080,
                ..Default::default()
            },
            Default::default(),
        );

        let shared = wrap_in_arc_mutex(LocalControlSharedData {
            get_prop_vals_cb,
            set_prop_vals_cb,
            properties: Vec::new(),
        });
        let shared_2 = shared.clone();

        protocomm.set_version_info("esp_local_ctrl/version", "v1.0".to_string());
        protocomm.set_security_endpoint("esp_local_ctrl/session");
        protocomm.register_endpoint(
            "esp_local_ctrl/control",
            Box::new(move |_, data| local_ctrl_callback(data, &shared_2)),
        );

        Self { protocomm, shared }
    }

    pub fn add_property(&mut self, name: String, property_type: u32, flags: u32) {
        self.shared.lock().unwrap().properties.push(Property {
            name,
            property_type,
            flags,
        });
    }
}

fn local_ctrl_callback(data: &[u8], shared: &WrappedInArcMutex<LocalControlSharedData>) -> Vec<u8> {
    let mut resp_buff = Vec::<u8>::new();
    let cmd_message = match LocalCtrlMessage::try_from(data) {
        Ok(payload) => payload,
        Err(_) => return resp_buff,
    };

    let resp_payload = match cmd_message.msg {
        LocalCtrlMsgType::TypeCmdGetPropertyCount => {
            cmd_handlers::get_property_count(cmd_message.payload, shared)
        }
        LocalCtrlMsgType::TypeCmdGetPropertyValues => {
            cmd_handlers::get_property_values(cmd_message.payload, shared)
        }
        LocalCtrlMsgType::TypeCmdSetPropertyValues => {
            cmd_handlers::set_property_values(cmd_message.payload, shared)
        }
        _ => unreachable!(),
    };

    let resp_msg = LocalCtrlMsgType::from(cmd_message.msg as i32 + 1);

    let out = LocalCtrlMessage {
        msg: resp_msg,
        payload: resp_payload,
    };

    let mut writer = Writer::new(&mut resp_buff);
    out.write_message(&mut writer).unwrap();

    resp_buff
}

mod cmd_handlers {
    use crate::{
        proto::{
            constants::Status,
            local_ctrl::{
                mod_LocalCtrlMessage::OneOfpayload, PropertyInfo, RespGetPropertyCount,
                RespGetPropertyValues, RespSetPropertyValues,
            },
        },
        utils::WrappedInArcMutex,
    };

    use super::LocalControlSharedData;

    pub fn get_property_count(
        _payload: OneOfpayload,
        shared: &WrappedInArcMutex<LocalControlSharedData>,
    ) -> OneOfpayload {
        let status: Status;
        let count: u32;

        match shared.lock() {
            Ok(shared) => {
                status = Status::Success;
                count = shared.properties.len() as u32;
            }
            Err(_) => {
                status = Status::InternalError;
                count = 0;
            }
        }

        OneOfpayload::resp_get_prop_count(RespGetPropertyCount { status, count })
    }

    pub fn get_property_values(
        payload: OneOfpayload,
        shared: &WrappedInArcMutex<LocalControlSharedData>,
    ) -> OneOfpayload {
        let status: Status;
        let mut props: Vec<PropertyInfo> = Vec::new();
        if let OneOfpayload::cmd_get_prop_vals(curr_payload) = payload {
            match shared.lock() {
                Ok(shared) => {
                    for property_index in curr_payload.indices {
                        if let Some(prop) = shared.properties.get(property_index as usize) {
                            let name = &prop.name;
                            let prop_type = prop.property_type;
                            let flags = prop.flags;
                            let value = (shared.get_prop_vals_cb)(name, prop_type, flags);

                            props.push(PropertyInfo {
                                status: Status::Success,
                                name: name.to_string(),
                                type_pb: prop_type,
                                value,
                                flags,
                            });
                        }
                    }
                    status = Status::Success;
                }
                Err(_) => {
                    status = Status::InternalError;
                    props = Default::default();
                }
            };
        } else {
            status = Status::InvalidArgument;
            props = Default::default();
        }

        OneOfpayload::resp_get_prop_vals(RespGetPropertyValues { status, props })
    }

    pub fn set_property_values(
        payload: OneOfpayload,
        shared: &WrappedInArcMutex<LocalControlSharedData>,
    ) -> OneOfpayload {
        let status;
        if let OneOfpayload::cmd_set_prop_vals(curr_payload) = payload {
            match shared.lock() {
                Ok(shared) => {
                    for property in curr_payload.props {
                        if let Some(prop) = shared.properties.get(property.index as usize) {
                            let name = &prop.name;
                            let prop_type = prop.property_type;
                            let flags = prop.flags;

                            (shared.set_prop_vals_cb)(name, prop_type, flags, property.value);
                        }
                    }

                    status = Status::Success;
                }
                Err(_) => status = Status::InternalError,
            }
        } else {
            status = Status::InvalidArgument;
        }

        OneOfpayload::resp_set_prop_vals(RespSetPropertyValues { status })
    }
}
