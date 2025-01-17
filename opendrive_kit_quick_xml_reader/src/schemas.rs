use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OpenDriveMap {
    pub road: Vec<Road>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Road {
    #[serde(rename = "@id")]
    pub id: i64,

    #[serde(rename = "@junction")]
    pub junction: i64,

    #[serde(rename = "@length")]
    pub length: f64,

    #[serde(rename = "@name")]
    pub name: Option<String>,

    #[serde(rename = "@rule")]
    pub rule: Option<String>,

    #[serde(rename = "planView")]
    pub plan_view: PlanView,

    #[serde(rename = "link")]
    pub link: Option<RoadLink>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PlanView {
    #[serde(default)]
    pub geometry: Vec<Geometry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Geometry {
    #[serde(rename = "@hdg")]
    pub hdg: f64,

    #[serde(rename = "@length")]
    pub length: f64,

    #[serde(rename = "@s")]
    pub s: f64,

    #[serde(rename = "@x")]
    pub x: f64,

    #[serde(rename = "@y")]
    pub y: f64,

    #[serde(rename = "$value")]
    pub sub_type: GeometryTypes,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RoadLink {
    #[serde(rename = "$value")]
    pub linkage: Vec<RoadLlinkPredecessorSuccessor>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RoadLlinkPredecessorSuccessor {
    Predecessor {
        #[serde(rename = "@contactPoint")]
        contact_point: Option<String>,
        #[serde(rename = "@elementDir")]
        element_dir: Option<String>,
        #[serde(rename = "@elementId")]
        element_id: String,
        #[serde(rename = "@elementS")]
        element_s: Option<f64>,
        #[serde(rename = "@elementType")]
        element_type: String,
    },
    Successor {
        #[serde(rename = "@contactPoint")]
        contact_point: Option<String>,
        #[serde(rename = "@elementDir")]
        element_dir: Option<String>,
        #[serde(rename = "@elementId")]
        element_id: String,
        #[serde(rename = "@elementS")]
        element_s: Option<f64>,
        #[serde(rename = "@elementType")]
        element_type: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GeometryTypes {
    Line,
    Spiral {
        #[serde(rename = "@curvEnd")]
        curv_end: f64,
        #[serde(rename = "@curvStart")]
        curv_start: f64,
    },
    Arc {
        #[serde(rename = "@curvature")]
        curvature: f64,
    },
    ParamPoly3 {
        #[serde(rename = "@aU")]
        a_u: f64,
        #[serde(rename = "@aV")]
        a_v: f64,
        #[serde(rename = "@bU")]
        b_u: f64,
        #[serde(rename = "@bV")]
        b_v: f64,
        #[serde(rename = "@cU")]
        c_u: f64,
        #[serde(rename = "@cV")]
        c_v: f64,
        #[serde(rename = "@dU")]
        d_u: f64,
        #[serde(rename = "@dV")]
        d_v: f64,
        #[serde(rename = "@pRange")]
        p_range: String,
    },

    #[serde(rename = "$text")]
    Text(String),
}
