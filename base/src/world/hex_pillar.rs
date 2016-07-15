
type HeightType = u16;

pub struct HexPillar {
    // plant_type
    // plant_seed
    // ground_type
    sections: Vec<PillarSection>,
    props: Vec<Prop>,
}

// from < to
pub struct PillarSection {
    // ground type
    //
    from: HeightType,
    to: HeightType,
}

pub struct Prop {
    baseline: HeightType,
    prop: PropType,
}

pub enum PropType {
}
