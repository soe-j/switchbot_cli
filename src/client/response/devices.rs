use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Device {
    device_id: String,
    device_name: String,
    device_type: String,
    hub_device_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct InfraredRemote {
    device_id: String,
    device_name: String,
    remote_type: String,
    hub_device_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct DevicesResponseBody {
    device_list: Vec<Device>,
    infrared_remote_list: Vec<InfraredRemote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DevicesResponse {
    status_code: i8,
    body: DevicesResponseBody,
}
