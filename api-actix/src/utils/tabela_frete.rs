use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAPA_FRETE: HashMap<i64, f64> = {
        let mut map = HashMap::new();
        map.insert(35, 0.0); // SP
        map.insert(31, 5.0); // MG
        map.insert(33, 5.0); // RJ
        map.insert(41, 5.0); // PR
        map.insert(50, 5.0); // MS
        map.insert(32, 10.0); // ES
        map.insert(29, 10.0); // BA
        map.insert(52, 10.0); // GO
        map.insert(51, 10.0); // MT
        map.insert(53, 10.0); // DF
        map.insert(42, 10.0); // SC
        map.insert(43, 15.0); // RS
        map.insert(11, 15.0); // RO
        map.insert(13, 15.0); // AM
        map.insert(15, 15.0); // PA
        map.insert(17, 15.0); // TO
        map.insert(22, 15.0); // PI
        map.insert(26, 15.0); // PE
        map.insert(27, 15.0); // AL
        map.insert(28, 15.0); // SE
        map.insert(12, 20.0); // AC
        map.insert(14, 20.0); // RR
        map.insert(16, 20.0); // AP
        map.insert(21, 20.0); // MA
        map.insert(23, 20.0); // CE
        map.insert(25, 20.0); // PB
        map.insert(24, 25.0); // RN
        map
    };
}