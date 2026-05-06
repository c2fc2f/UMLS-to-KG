//! Module of a Saver which saves every relation of semantic type

use std::path::Path;
use umls::semantic_network::relation::models::{
    RelationType, SemanticTypeRelationship,
};

use crate::{saver::Writer, writer};

/// macro to generate the saver
macro_rules! generate_rel_saver {
    ($struct_name:ident, { $($variant:ident ; $file:literal ; $field:ident),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $struct_name {
            $(
                #[doc = concat!("CSV Writer for ", stringify!($variant), " Relation")]
                pub $field: Writer,
            )*
        }

        impl $struct_name {
            pub async fn new(dir: &Path) -> std::io::Result<Self> {
                Ok(Self {
                    $(
                        $field: writer!(
                            dir,
                            concat!("SN-", $file),
                            [
                                ":START_ID(UMLSSemanticNetwork)",
                                ":END_ID(UMLSSemanticNetwork)"
                            ]
                        ),
                    )*
                })
            }

            /// Flush every CSV file
            pub async fn flush(mut self) -> std::io::Result<()> {
                $(
                    self.$field.flush()?;
                )*
                Ok(())
            }

            /// Save one record
            pub async fn save_record(
                &mut self,
                rel: SemanticTypeRelationship,
            ) -> std::io::Result<()> {
                let file: &mut Writer = match rel.relation {
                    $(
                        RelationType::$variant => &mut self.$field,
                    )*
                };

                file.write_record([
                    &rel.subject,
                    &rel.object,
                ])?;

                Ok(())
            }
        }
    };
}

generate_rel_saver!(SemTypeRelSaver, {
    PhysicallyRelatedTo      ; "PHYSICALLY_RELATED_TO"     ; physically_related_to,
    PartOf                   ; "PART_OF"                    ; part_of,
    Contains                 ; "CONTAINS"                  ; contains,
    LocationOf               ; "LOCATION_OF"               ; location_of,
    TemporallyRelatedTo      ; "TEMPORALLY_RELATED_TO"      ; temporally_related_to,
    CoOccursWith             ; "CO_OCCURS_WITH"             ; co_occurs_with,
    Precedes                 ; "PRECEDES"                  ; precedes,
    FunctionallyRelatedTo    ; "FUNCTIONALLY_RELATED_TO"    ; functionally_related_to,
    ProcessOf                ; "PROCESS_OF"                ; process_of,
    CarriesOut               ; "CARRIES_OUT"               ; carries_out,
    InteractsWith            ; "INTERACTS_WITH"            ; interacts_with,
    Practices                ; "PRACTICES"                 ; practices,
    Produces                 ; "PRODUCES"                  ; produces,
    Exhibits                 ; "EXHIBITS"                  ; exhibits,
    Disrupts                 ; "DISRUPTS"                  ; disrupts,
    Causes                   ; "CAUSES"                    ; causes,
    Prevents                 ; "PREVENTS"                  ; prevents,
    Complicates              ; "COMPLICATES"               ; complicates,
    ManifestationOf          ; "MANIFESTATION_OF"          ; manifestation_of,
    Affects                  ; "AFFECTS"                   ; affects,
    OccursIn                 ; "OCCURS_IN"                 ; occurs_in,
    Manages                  ; "MANAGES"                   ; manages,
    Treats                   ; "TREATS"                    ; treats,
    Uses                     ; "USES"                      ; uses,
    Indicates                ; "INDICATES"                 ; indicates,
    ResultOf                 ; "RESULT_OF"                 ; result_of,
    ConceptuallyRelatedTo    ; "CONCEPTUALLY_RELATED_TO"    ; conceptually_related_to,
    PropertyOf               ; "PROPERTY_OF"               ; property_of,
    ConceptualPartOf         ; "CONCEPTUAL_PART_OF"         ; conceptual_part_of,
    EvaluationOf             ; "EVALUATION_OF"             ; evaluation_of,
    Measures                 ; "MEASURES"                  ; measures,
    Diagnoses                ; "DIAGNOSES"                 ; diagnoses,
    AssessesEffectOf         ; "ASSESSES_EFFECT_OF"         ; assesses_effect_of,
    IssueIn                  ; "ISSUE_IN"                  ; issue_in,
    AssociatedWith           ; "ASSOCIATED_WITH"           ; associated_with,
    ConsistsOf               ; "CONSISTS_OF"               ; consists_of,
    AdjacentTo               ; "ADJACENT_TO"               ; adjacent_to,
    ConnectedTo              ; "CONNECTED_TO"              ; connected_to,
    Interconnects            ; "INTERCONNECTS"             ; interconnects,
    Surrounds                ; "SURROUNDS"                 ; surrounds,
    Traverses                ; "TRAVERSES"                 ; traverses,
    DerivativeOf             ; "DERIVATIVE_OF"             ; derivative_of,
    DevelopmentalFormOf      ; "DEVELOPMENTAL_FORM_OF"      ; developmental_form_of,
    DegreeOf                 ; "DEGREE_OF"                 ; degree_of,
    MeasurementOf            ; "MEASUREMENT_OF"            ; measurement_of,
    MethodOf                 ; "METHOD_OF"                 ; method_of,
    Isa                      ; "IS_A"                       ; isa,
    BringsAbout              ; "BRINGS_ABOUT"              ; brings_about,
    Performs                 ; "PERFORMS"                  ; performs,
    SpatiallyRelatedTo       ; "SPATIALLY_RELATED_TO"       ; spatially_related_to,
    Analyzes                 ; "ANALYZES"                  ; analyzes,
    BranchOf                 ; "BRANCH_OF"                 ; branch_of,
    TributaryOf              ; "TRIBUTARY_OF"              ; tributary_of,
    IngredientOf             ; "INGREDIENT_OF"             ; ingredient_of,
});
