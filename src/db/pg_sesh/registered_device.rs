use uuid::Uuid;

use crate::db::{curr_time::get_current_time, pg_sesh::Sesh, types::registered_device::{DeviceInfo, RegisteredDevice}};

#[allow(dead_code)]
impl Sesh<'_> {
    pub async fn create_registered_device(&self, device: &DeviceInfo) -> RegisteredDevice {
        let id = Uuid::now_v7();
        let result = self
            .query(
                RegisteredDevice::create_statement(),
                &[
                    &id,
                    &device.device_name,
                    &device.software,
                    &device.webpage,
                    &device.redirect_url,
                    &get_current_time(),
                ],
            )
            .await
            .expect("failed to create registered device")
            .pop()
            .expect("creating registered device returned nothing");
        result.into()
    }
    pub async fn get_registered_device(&self, device_id: &Uuid) -> Option<RegisteredDevice> {
        let result = self
            .query(RegisteredDevice::read_statement(), &[device_id])
            .await
            .expect("failed to fetch registered device")
            .pop();
        result.map(|x| x.into())
    }
    pub async fn delete_registered_device(&self, device_id: &Uuid) {
        let _result = self
            .query(RegisteredDevice::delete_statement(), &[device_id])
            .await
            .expect("failed to delete registered device");
    }
}