use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::{Deserialize, Serialize};
use tokio::time;

#[tokio::main]
async fn main() {
    // let brokers = "b-1.gsmdrivermskclust.9qqjbi.c3.kafka.ap-southeast-1.amazonaws.com:9096,b-2.gsmdrivermskclust.9qqjbi.c3.kafka.ap-southeast-1.amazonaws.com:9096";
    let brokers = "localhost:9092";
    let producer: &FutureProducer = &ClientConfig::new()
        // .set("security.protocol", "SASL_PLAINTEXT")
        // .set("sasl.mechanisms", "PLAIN")
        // .set("sasl.username", "gsm-driver-msk-dev")
        // .set("sasl.password", ";^x*C+v53=8d")
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let sample = r#"
{"zone_id":"07df6cd7-0145-4453-94b9-cf967925fd64","order_id":"01HW9R9B40NHYZ01M0WMVP80GX","service_id":6,"customer_id":"ba65a967-729e-4285-abc5-9c8495b6c1f3","order_location":{"longitude":106.77782,"latitude":10.8057},"order_time":1714019413,"order_status":"FINDING","travel_mode":"Car","service_type":"RIDE-TRIP","expired_at":1714019713,"capacity_point":4,"total_fee":157000,"commission":122914,"vehicle_models":["VF5","VF-E34","VF6","VFE34","VF7"],"driver_lock_time":1714019413,"currency":"VND","is_cancel_by_driver":false,"created_by":"ba65a967-729e-4285-abc5-9c8495b6c1f3","polylines":[{"start":[106.777797,10.805694],"end":[106.76462,10.76831],"polyline":"qn}`Ag`vjSOBDPl@M|FmAzA]fB_@v@QtB_@dAQFCnCg@pEy@tBc@~Co@NC@AhAUREvIeB@v@HdI@Z@Z@TVpJFrAJT@bA@|@\\z`@@dBD`BJnGF`C@^Bh@DvANnCL~BFv@Bb@Df@TnCNfBHv@BZRdBHn@L|@PrAVdBVzAJn@RfAx@|EBPBRFt@Fr@Dl@JdAJhAJfAPbAXbAHb@tApGJ`@FVBLDJD^?F^r@Xd@NZd@~@PRALP`@L~@b@FpBRTBVBTBL@lCZd@FZDd@Hp@Lr@R`AZt@\\RJb@Rz@h@z@`@b@XXHVBb@?PQLKZWbAs@dAi@RMf@W\\UpK}IlHmGJKhA_AFOxJwHLI|GsF\\YNMfEeDnGoFXUDGb@]x@w@LI|@u@z@u@d@a@lAcAHIRQpAuAt@qAVaAFUD]FsBBU?q@b@N`@^~@fArAoAfA_AxDsDxAsAlAjA|@_A~@aAfDbD"}],"actor_role":"system","has_insurance":true,"insurance_addon_id":6,"insurance_fee":0,"total_pay":157000,"source":"user_app","commission_drivers":[{"sap_contract_type":"bike","total_commission":43851,"total_commission_display":"43.851đ"},{"sap_contract_type":"car","total_commission":122914,"total_commission_display":"122.914đ"},{"sap_contract_type":"car_emddi","total_commission":54250,"total_commission_display":"54.250đ"},{"sap_contract_type":"car_platform","total_commission":51135,"total_commission_display":"51.135đ"},{"sap_contract_type":"partner","total_commission":2000,"total_commission_display":"2.000đ"}]}
        "#;
    let mut order = serde_json::from_str::<Order>(sample).unwrap();
    let a = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    order.expired_at += a;
    order.order_status = "CANCELLED".into();

    let item = 10000;
    for i in 0..item {
        let mut a = order.clone();
        a.order_id = format!("order-{i}");
        let order_payload = serde_json::to_string(&a).unwrap();
        producer
            .send(
                FutureRecord::to("order-service.order-status")
                    .key(order.order_id.as_str())
                    .payload(&order_payload),
                Duration::from_millis(1000),
            )
            .await
            .unwrap();
    }

    // time::sleep(Duration::from_secs(10)).await;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "zone_id")]
    pub zone_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "service_id")]
    pub service_id: i64,
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    #[serde(rename = "order_location")]
    pub order_location: OrderLocation,
    #[serde(rename = "order_time")]
    pub order_time: i64,
    #[serde(rename = "order_status")]
    pub order_status: String,
    #[serde(rename = "travel_mode")]
    pub travel_mode: String,
    #[serde(rename = "service_type")]
    pub service_type: String,
    #[serde(rename = "expired_at")]
    pub expired_at: i64,
    #[serde(rename = "capacity_point")]
    pub capacity_point: i64,
    #[serde(rename = "total_fee")]
    pub total_fee: i64,
    pub commission: i64,
    #[serde(rename = "vehicle_models")]
    pub vehicle_models: Vec<String>,
    #[serde(rename = "driver_lock_time")]
    pub driver_lock_time: i64,
    pub currency: String,
    #[serde(rename = "is_cancel_by_driver")]
    pub is_cancel_by_driver: bool,
    #[serde(rename = "created_by")]
    pub created_by: String,
    pub polylines: Vec<Polyline>,
    #[serde(rename = "actor_role")]
    pub actor_role: String,
    #[serde(rename = "has_insurance")]
    pub has_insurance: bool,
    #[serde(rename = "insurance_addon_id")]
    pub insurance_addon_id: i64,
    #[serde(rename = "insurance_fee")]
    pub insurance_fee: i64,
    #[serde(rename = "total_pay")]
    pub total_pay: i64,
    pub source: String,
    #[serde(rename = "commission_drivers")]
    pub commission_drivers: Vec<CommissionDriver>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLocation {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Polyline {
    pub start: Vec<f64>,
    pub end: Vec<f64>,
    pub polyline: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommissionDriver {
    #[serde(rename = "sap_contract_type")]
    pub sap_contract_type: String,
    #[serde(rename = "total_commission")]
    pub total_commission: i64,
    #[serde(rename = "total_commission_display")]
    pub total_commission_display: String,
}
