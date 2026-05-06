//! Module regrouping datatype

use serde::{Deserialize, Serialize};

/// Semantic Type Relationship (SRSTRE1.RRF)
///
/// This table expresses binary relations between Semantic Types.
/// These are ordered pairs where all relations have been fully inherited.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticTypeRelationship {
    /// UI: Argument 1 UI
    /// Represents the "Subject" of the binary relation.
    pub subject: String,

    /// UI: Relation UI
    /// Represents the link between Argument 1 and Argument 2.
    pub relation: RelationType,

    /// UI: Argument 2 UI
    /// Represents the "Object" of the binary relation.
    pub object: String,
}

/// Relation Type (UI)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    /// Related by virtue of some physical attribute or characteristic.
    #[serde(rename = "T132")]
    PhysicallyRelatedTo,
    /// Composes, with one or more other physical units, some larger whole.
    #[serde(rename = "T133")]
    PartOf,
    /// Holds or is the receptacle for fluids or other substances.
    #[serde(rename = "T134")]
    Contains,
    /// The position, site, or region of an entity or the site of a process.
    #[serde(rename = "T135")]
    LocationOf,
    /// Related in time by preceding, co-occuring with, or following.
    #[serde(rename = "T136")]
    TemporallyRelatedTo,
    /// Occurs at the same time as, together with, or jointly.
    #[serde(rename = "T137")]
    CoOccursWith,
    /// Occurs earlier in time.
    #[serde(rename = "T138")]
    Precedes,
    /// Related by the carrying out of some function or activity.
    #[serde(rename = "T139")]
    FunctionallyRelatedTo,
    /// Action, function, or state of.
    #[serde(rename = "T140")]
    ProcessOf,
    /// Executes a function or performs a procedure or activity.
    #[serde(rename = "T141")]
    CarriesOut,
    /// Acts, functions, or operates together with.
    #[serde(rename = "T142")]
    InteractsWith,
    /// Performs habitually or customarily.
    #[serde(rename = "T143")]
    Practices,
    /// Brings forth, generates or creates.
    #[serde(rename = "T144")]
    Produces,
    /// Shows or demonstrates.
    #[serde(rename = "T145")]
    Exhibits,
    /// Alters or influences an already existing condition, state, or
    /// situation. Produces a negative effect on.
    #[serde(rename = "T146")]
    Disrupts,
    /// Brings about a condition or an effect.
    #[serde(rename = "T147")]
    Causes,
    /// Stops, hinders or eliminates an action or condition.
    #[serde(rename = "T148")]
    Prevents,
    /// Causes to become more severe or complex or results in adverse effects.
    #[serde(rename = "T149")]
    Complicates,
    /// That part of a phenomenon which is directly observable or expressed.
    #[serde(rename = "T150")]
    ManifestationOf,
    /// Produces a direct effect on.
    #[serde(rename = "T151")]
    Affects,
    /// Takes place in or happens under given conditions.
    #[serde(rename = "T152")]
    OccursIn,
    /// Administers, or contributes to the care of an individual or group.
    #[serde(rename = "T153")]
    Manages,
    /// Applies a remedy with the object of effecting a cure.
    #[serde(rename = "T154")]
    Treats,
    /// Employs in the carrying out of some activity.
    #[serde(rename = "T155")]
    Uses,
    /// Gives evidence for the presence at some time of an entity or process.
    #[serde(rename = "T156")]
    Indicates,
    /// The condition, product, or state occurring as a consequence.
    #[serde(rename = "T157")]
    ResultOf,
    /// Related by some abstract concept, thought, or idea.
    #[serde(rename = "T158")]
    ConceptuallyRelatedTo,
    /// Characteristic of, or quality of.
    #[serde(rename = "T159")]
    PropertyOf,
    /// Conceptually a portion, division, or component of some larger whole.
    #[serde(rename = "T160")]
    ConceptualPartOf,
    /// Judgment of the value or degree of some attribute or process.
    #[serde(rename = "T161")]
    EvaluationOf,
    /// Ascertains or marks the dimensions, quantity, degree, or capacity of.
    #[serde(rename = "T162")]
    Measures,
    /// Distinguishes or identifies the nature or characteristics of.
    #[serde(rename = "T163")]
    Diagnoses,
    /// Analyzes the influence or consequences of the function or action of.
    #[serde(rename = "T164")]
    AssessesEffectOf,
    /// Is an issue in or a point of discussion, study, debate, or dispute.
    #[serde(rename = "T165")]
    IssueIn,
    /// Has a significant or salient relationship to.
    #[serde(rename = "T166")]
    AssociatedWith,
    /// Is structurally made up of in whole or in part of some material.
    #[serde(rename = "T172")]
    ConsistsOf,
    /// Close to, near or abutting another physical unit.
    #[serde(rename = "T173")]
    AdjacentTo,
    /// Directly attached to another physical unit.
    #[serde(rename = "T174")]
    ConnectedTo,
    /// Serves to link or join together two or more other physical units.
    #[serde(rename = "T175")]
    Interconnects,
    /// Establishes the boundaries for another physical structure.
    #[serde(rename = "T176")]
    Surrounds,
    /// Crosses or extends across another physical structure or area.
    #[serde(rename = "T177")]
    Traverses,
    /// Substance structurally related to another or that can be made from it.
    #[serde(rename = "T178")]
    DerivativeOf,
    /// An earlier stage in the individual maturation of.
    #[serde(rename = "T179")]
    DevelopmentalFormOf,
    /// The relative intensity of a process or quality.
    #[serde(rename = "T180")]
    DegreeOf,
    /// The dimension, quantity, or capacity determined by measuring.
    #[serde(rename = "T182")]
    MeasurementOf,
    /// The manner and sequence of events in performing an act or procedure.
    #[serde(rename = "T183")]
    MethodOf,
    /// The basic hierarchical link in the Network.
    #[serde(rename = "T186")]
    Isa,
    /// Acts on or influences an entity.
    #[serde(rename = "T187")]
    BringsAbout,
    /// Executes, accomplishes, or achieves an activity.
    #[serde(rename = "T188")]
    Performs,
    /// Related by place or region.
    #[serde(rename = "T189")]
    SpatiallyRelatedTo,
    /// Studies or examines using established methods.
    #[serde(rename = "T193")]
    Analyzes,
    /// Arises from the division of.
    #[serde(rename = "T198")]
    BranchOf,
    /// Merges with.
    #[serde(rename = "T199")]
    TributaryOf,
    /// Is a component of, as in a constituent of a preparation.
    #[serde(rename = "T202")]
    IngredientOf,
}
