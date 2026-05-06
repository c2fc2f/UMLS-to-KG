//! Module regrouping datatype

use serde::{Deserialize, Deserializer, Serialize};

/// Represents a single record from the MRCONSO.RRF file (Concept Names and
/// Sources).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CoNSoRecord {
    /// Unique identifier for concept (CUI)
    pub cui: String,

    /// Language of term (LAT)
    pub lat: LanguageOfTerms,

    /// Term status (TS)
    pub ts: TermStatus,

    /// Unique identifier for term (LUI)
    pub lui: String,

    /// String type (STT)
    pub stt: StringType,

    /// Unique identifier for string (SUI)
    pub sui: String,

    /// Atom status - preferred (Y) or not (N) for this string within this
    /// concept (ISPREF)
    #[serde(rename = "ISPREF", deserialize_with = "deserialize_yes_no")]
    pub is_pref: bool,

    /// Unique identifier for atom - variable length field, 8 or 9 characters
    /// (AUI)
    pub aui: String,

    /// Source asserted atom identifier [optional] (SAUI)
    pub saui: Option<String>,

    /// Source asserted concept identifier [optional] (SCUI)
    pub scui: Option<String>,

    /// Source asserted descriptor identifier [optional] (SDUI)
    pub sdui: Option<String>,

    /// Abbreviated source name (SAB). Max length 20 alphanumeric characters.
    pub sab: String,

    /// Abbreviation for term type in source vocabulary (e.g., PN, CD) (TTY)
    pub tty: TermType,

    /// Most useful source asserted identifier (CODE)
    pub code: String,

    /// The actual string/term (STR)
    #[serde(rename = "STR")]
    pub string: String,

    /// Source restriction level (SRL)
    pub srl: SourceRestrictionLevel,

    /// Suppressible flag.
    pub suppress: SuppressStatus,
}

/// Language of Terms (LAT)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LanguageOfTerms {
    /// Arabic
    Ara,
    /// Basque
    Baq,
    /// Chinese
    Chi,
    /// Czech
    Cze,
    /// Danish
    Dan,
    /// Dutch
    Dut,
    /// English
    Eng,
    /// Estonian
    Est,
    /// Finnish
    Fin,
    /// French
    Fre,
    /// German
    Ger,
    /// Greek
    Gre,
    /// Hebrew
    Heb,
    /// Hungarian
    Hun,
    /// Icelandic
    Isl,
    /// Italian
    Ita,
    /// Japanese
    Jpn,
    /// Korean
    Kor,
    /// Latvian
    Lav,
    /// Lithuanian
    Lit,
    /// Norwegian
    Nor,
    /// Polish
    Pol,
    /// Portuguese
    Por,
    /// Russian
    Rus,
    /// Croatian
    Scr,
    /// Slovak
    Slk,
    /// Slovenian
    Slv,
    /// Spanish
    Spa,
    /// Swedish
    Swe,
    /// Turkish
    Tur,
    /// Ukrainian
    Ukr,
}

/// Term Status (TS)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermStatus {
    /// Preferred LUI of the CUI
    #[serde(rename = "P")]
    Preferred,

    /// Non-Preferred LUI of the CUI
    #[serde(rename = "S")]
    NonPreferred,

    /// Preferred LUI of the CUI, suppressible
    #[serde(rename = "p")]
    PreferredSuppressible,

    /// Non-Preferred LUI of the CUI, suppressible
    #[serde(rename = "s")]
    NonPreferredSuppressible,
}

/// String Type (STT)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StringType {
    /// Preferred form of term
    #[serde(rename = "PF")]
    PreferredForm,

    /// Case and word-order variant of the preferred form
    #[serde(rename = "VCW")]
    CaseAndWordOrderVariant,

    /// Case variant of the preferred form
    #[serde(rename = "VC")]
    CaseVariant,

    /// Variant of the preferred form
    #[serde(rename = "VO")]
    Variant,

    /// Word-order variant of the preferred form
    #[serde(rename = "VW")]
    WordOrderVariant,
}

/// Term Type in Source (TTY)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermType {
    /// Attribute type abbreviation
    #[serde(rename = "AA")]
    AttributeTypeAbbreviation,
    /// Abbreviation in any source vocabulary
    #[serde(rename = "AB")]
    Abbreviation,
    /// Acronym
    #[serde(rename = "ACR")]
    Acronym,
    /// Activities
    #[serde(rename = "AC")]
    Activities,
    /// Adjective
    #[serde(rename = "AD")]
    Adjective,
    /// Short form of modifier
    #[serde(rename = "AM")]
    ShortFormModifier,
    /// Attribute type synonym
    #[serde(rename = "AS")]
    AttributeTypeSynonym,
    /// Attribute type
    #[serde(rename = "AT")]
    AttributeType,
    /// Fully-specified drug brand name that can be prescribed
    #[serde(rename = "BD")]
    FullySpecifiedDrugBrandNamePrescribable,
    /// Fully-specified drug brand name that can not be prescribed
    #[serde(rename = "BN")]
    FullySpecifiedDrugBrandNameNonPrescribable,
    /// Branded Drug Delivery Device
    #[serde(rename = "BPCK")]
    BrandedDrugDeliveryDevice,
    /// Binding realm
    #[serde(rename = "BR")]
    BindingRealm,
    /// ISO 3166-1 standard country code in alpha-2 (two-letter) format
    #[serde(rename = "CA2")]
    IsoCountryCodeAlpha2,
    /// ISO 3166-1 standard country code in alpha-3 (three-letter) format
    #[serde(rename = "CA3")]
    IsoCountryCodeAlpha3,
    /// Chemical code name
    #[serde(rename = "CCN")]
    ChemicalCodeName,
    /// Trimmed ICPC component process
    #[serde(rename = "CC")]
    TrimmedIcpcComponentProcess,
    /// Clinical drug name in abbreviated format
    #[serde(rename = "CDA")]
    ClinicalDrugAbbreviated,
    /// Clinical drug name in concatenated format (NDDF)
    #[serde(rename = "CDC")]
    ClinicalDrugConcatenated,
    /// Clinical drug name in delimited format
    #[serde(rename = "CDD")]
    ClinicalDrugDelimited,
    /// Concept domain
    #[serde(rename = "CDO")]
    ConceptDomain,
    /// Clinical Drug
    #[serde(rename = "CD")]
    ClinicalDrug,
    /// Entry term for a Supplementary Concept
    #[serde(rename = "CE")]
    EntryTermSupplementaryConcept,
    /// Chemical structure name
    #[serde(rename = "CHN")]
    ChemicalStructureName,
    /// Class
    #[serde(rename = "CL")]
    Class,
    /// Common name
    #[serde(rename = "CMN")]
    CommonName,
    /// LOINC official component name
    #[serde(rename = "CN")]
    LoincOfficialComponentName,
    /// Component name (hierarchical terms, as opposed to LOINC analytes)
    #[serde(rename = "CO")]
    HierarchicalComponentName,
    /// Concept property
    #[serde(rename = "CPR")]
    ConceptProperty,
    /// ICPC component process (in original form)
    #[serde(rename = "CP")]
    IcpcComponentProcess,
    /// Concept relationship
    #[serde(rename = "CR")]
    ConceptRelationship,
    /// Chemical Structure Name
    #[serde(rename = "CSN")]
    ChemicalStructureNameUpper,
    /// Code system
    #[serde(rename = "CSY")]
    CodeSystem,
    /// Short component process in ICPC, i.e. include some abbreviations
    #[serde(rename = "CS")]
    ShortComponentProcessIcpc,
    /// Common usage
    #[serde(rename = "CU")]
    CommonUsage,
    /// Content view
    #[serde(rename = "CV")]
    ContentView,
    /// Component, with abbreviations expanded.
    #[serde(rename = "CX")]
    ComponentExpanded,
    /// Diagnostic criteria for ICD10 code
    #[serde(rename = "DC10")]
    DiagnosticCriteriaIcd10,
    /// Diagnostic criteria for ICD9 code
    #[serde(rename = "DC9")]
    DiagnosticCriteriaIcd9,
    /// Descriptor entry version
    #[serde(rename = "DEV")]
    DescriptorEntryVersion,
    /// Descriptor
    #[serde(rename = "DE")]
    Descriptor,
    /// Dose Form Group
    #[serde(rename = "DFG")]
    DoseFormGroup,
    /// Dose Form
    #[serde(rename = "DF")]
    DoseForm,
    /// Disease name
    #[serde(rename = "DI")]
    DiseaseName,
    /// Display Name
    #[serde(rename = "DN")]
    DisplayName,
    /// Domain
    #[serde(rename = "DO")]
    Domain,
    /// Drug Product
    #[serde(rename = "DP")]
    DrugProduct,
    /// Descriptor sort version
    #[serde(rename = "DSV")]
    DescriptorSortVersion,
    /// Short form of descriptor
    #[serde(rename = "DS")]
    ShortFormDescriptor,
    /// Definitional term connection to Dorland's or Metathesaurus
    #[serde(rename = "DT")]
    DefinitionalTerm,
    /// Print entry term
    #[serde(rename = "EP")]
    PrintEntryTerm,
    /// Equivalent name
    #[serde(rename = "EQ")]
    EquivalentName,
    /// Short form of entry term
    #[serde(rename = "ES")]
    ShortFormEntryTerm,
    /// Entry Term Alias
    #[serde(rename = "ETAL")]
    EntryTermAlias,
    /// Entry term, consumer friendly description
    #[serde(rename = "ETCF")]
    EntryTermConsumerFriendly,
    /// Entry term, clinician description
    #[serde(rename = "ETCLIN")]
    EntryTermClinician,
    /// Entry term
    #[serde(rename = "ET")]
    EntryTerm,
    /// Expanded form of entry term
    #[serde(rename = "EX")]
    ExpandedFormEntryTerm,
    /// Foreign brand name
    #[serde(rename = "FBD")]
    ForeignBrandName,
    /// Finding name
    #[serde(rename = "FI")]
    FindingName,
    /// Full form of descriptor
    #[serde(rename = "FN")]
    FullFormDescriptor,
    /// Foreign Synonym
    #[serde(rename = "FSY")]
    ForeignSynonym,
    /// Global period
    #[serde(rename = "GLP")]
    GlobalPeriod,
    /// Generic drug name
    #[serde(rename = "GN")]
    GenericDrugName,
    /// Goal
    #[serde(rename = "GO")]
    Goal,
    /// Generic Drug Delivery Device
    #[serde(rename = "GPCK")]
    GenericDrugDeliveryDevice,
    /// Glossary term
    #[serde(rename = "GT")]
    GlossaryTerm,
    /// Hierarchical class
    #[serde(rename = "HC")]
    HierarchicalClass,
    /// Hierarchical descriptor
    #[serde(rename = "HD")]
    HierarchicalDescriptor,
    /// Japanese High Level Group Term (kana1)
    #[serde(rename = "HGJKN1")]
    JapaneseHighLevelGroupTermKana1,
    /// Japanese High Level Group Term (kana)
    #[serde(rename = "HGJKN")]
    JapaneseHighLevelGroupTermKana,
    /// High Level Group Term
    #[serde(rename = "HG")]
    HighLevelGroupTerm,
    /// Short or alternate version of hierarchical term
    #[serde(rename = "HS")]
    ShortHierarchicalTerm,
    /// Japanese Hierarchical term (kana1)
    #[serde(rename = "HTJKN1")]
    JapaneseHierarchicalTermKana1,
    /// Japanese Hierarchical term (kana)
    #[serde(rename = "HTJKN")]
    JapaneseHierarchicalTermKana,
    /// HL7 Table Name
    #[serde(rename = "HTN")]
    Hl7TableName,
    /// Hierarchical term
    #[serde(rename = "HT")]
    HierarchicalTerm,
    /// Expanded version of short hierarchical term
    #[serde(rename = "HX")]
    ExpandedShortHierarchicalTerm,
    /// Nursing indicator
    #[serde(rename = "ID")]
    NursingIndicator,
    /// Name for an ingredient
    #[serde(rename = "IN")]
    IngredientName,
    /// Obsolete Synonym
    #[serde(rename = "IS")]
    ObsoleteSynonym,
    /// Index term
    #[serde(rename = "IT")]
    IndexTerm,
    /// Intervention categories
    #[serde(rename = "IVC")]
    InterventionCategories,
    /// Intervention
    #[serde(rename = "IV")]
    Intervention,
    /// LOINC answer
    #[serde(rename = "LA")]
    LoincAnswer,
    /// Long common name
    #[serde(rename = "LC")]
    LongCommonName,
    /// LOINC group
    #[serde(rename = "LG")]
    LoincGroup,
    /// Japanese Lower Level Term (kana1)
    #[serde(rename = "LLTJKN1")]
    JapaneseLowerLevelTermKana1,
    /// Japanese Lower Level Term (kana)
    #[serde(rename = "LLTJKN")]
    JapaneseLowerLevelTermKana,
    /// Lower Level Term
    #[serde(rename = "LLT")]
    LowerLevelTerm,
    /// LOINC official fully specified name
    #[serde(rename = "LN")]
    LoincOfficialFullySpecifiedName,
    /// Obsolete official fully specified name
    #[serde(rename = "LO")]
    ObsoleteOfficialFullySpecifiedName,
    /// LOINC parts display name
    #[serde(rename = "LPDN")]
    LoincPartsDisplayName,
    /// LOINC parts name
    #[serde(rename = "LPN")]
    LoincPartsName,
    /// Expanded system/sample type
    #[serde(rename = "LS")]
    ExpandedSystemSampleType,
    /// Linguistic variant display name
    #[serde(rename = "LVDN")]
    LinguisticVariantDisplayName,
    /// Lexical variant
    #[serde(rename = "LV")]
    LexicalVariant,
    /// CCS multi-level diagnosis categories
    #[serde(rename = "MD")]
    CcsMultiLevelDiagnosisCategories,
    /// Main heading
    #[serde(rename = "MH")]
    MainHeading,
    /// Name for a multi-ingredient
    #[serde(rename = "MIN")]
    MultiIngredientName,
    /// Preferred names of modifiers
    #[serde(rename = "MP")]
    PreferredNamesOfModifiers,
    /// Multum names of branded and generic supplies or supplements
    #[serde(rename = "MS")]
    MultumNames,
    /// MTH acronym
    #[serde(rename = "MTH_ACR")]
    MthAcronym,
    /// MTH Component, with abbreviations expanded.
    #[serde(rename = "MTH_CN")]
    MthComponentExpanded,
    /// Metathesaurus entry term
    #[serde(rename = "MTH_ET")]
    MthEntryTerm,
    /// MTH Full form of descriptor
    #[serde(rename = "MTH_FN")]
    MthFullFormDescriptor,
    /// MTH High Level Group Term
    #[serde(rename = "MTH_HG")]
    MthHighLevelGroupTerm,
    /// MTH Hierarchical term
    #[serde(rename = "MTH_HT")]
    MthHierarchicalTerm,
    /// MTH Hierarchical term expanded
    #[serde(rename = "MTH_HX")]
    MthHierarchicalTermExpanded,
    /// Metathesaurus-supplied form of obsolete synonym
    #[serde(rename = "MTH_IS")]
    MthObsoleteSynonym,
    /// MTH Lower Level Term
    #[serde(rename = "MTH_LLT")]
    MthLowerLevelTerm,
    /// MTH Official fully specified name with expanded abbreviations
    #[serde(rename = "MTH_LN")]
    MthOfficialFullySpecifiedName,
    /// MTH Expanded LOINC obsolete fully specified name
    #[serde(rename = "MTH_LO")]
    MthExpandedLoincObsoleteName,
    /// Metathesaurus-supplied form of obsolete active fully specified name
    #[serde(rename = "MTH_OAF")]
    MthObsoleteActiveFullySpecifiedName,
    /// Metathesaurus-supplied form of obsolete active preferred term
    #[serde(rename = "MTH_OAP")]
    MthObsoleteActivePreferredTerm,
    /// Metathesaurus-supplied form of obsolete active synonym
    #[serde(rename = "MTH_OAS")]
    MthObsoleteActiveSynonym,
    /// Metathesaurus obsolete entry term
    #[serde(rename = "MTH_OET")]
    MthObsoleteEntryTerm,
    /// Metathesaurus-supplied form of obsolete fully specified name
    #[serde(rename = "MTH_OF")]
    MthObsoleteFullySpecifiedName,
    /// MTH Non-current Lower Level Term
    #[serde(rename = "MTH_OL")]
    MthNonCurrentLowerLevelTerm,
    /// Metathesaurus obsolete preferred term, natural language form
    #[serde(rename = "MTH_OPN")]
    MthObsoletePreferredTermNatural,
    /// Metathesaurus obsolete preferred term
    #[serde(rename = "MTH_OP")]
    MthObsoletePreferredTerm,
    /// MTH System-organ class
    #[serde(rename = "MTH_OS")]
    MthSystemOrganClass,
    /// Metathesaurus-supplied form of British preferred term
    #[serde(rename = "MTH_PTGB")]
    MthBritishPreferredTerm,
    /// Metathesaurus preferred term, natural language form
    #[serde(rename = "MTH_PTN")]
    MthPreferredTermNatural,
    /// Metathesaurus preferred term
    #[serde(rename = "MTH_PT")]
    MthPreferredTerm,
    /// RxNorm Created BD
    #[serde(rename = "MTH_RXN_BD")]
    MthRxnormCreatedBd,
    /// RxNorm Created CDC
    #[serde(rename = "MTH_RXN_CDC")]
    MthRxnormCreatedCdc,
    /// RxNorm Created CD
    #[serde(rename = "MTH_RXN_CD")]
    MthRxnormCreatedCd,
    /// RxNorm Created DP
    #[serde(rename = "MTH_RXN_DP")]
    MthRxnormCreatedDp,
    /// MTH Sign or symptom of
    #[serde(rename = "MTH_SI")]
    MthSignSymptom,
    /// Metathesaurus version of Standardised MedDRA Query
    #[serde(rename = "MTH_SMQ")]
    MthMeddraQuery,
    /// Metathesaurus-supplied form of British synonym
    #[serde(rename = "MTH_SYGB")]
    MthBritishSynonym,
    /// MTH Designated synonym
    #[serde(rename = "MTH_SY")]
    MthDesignatedSynonym,
    /// Multi-level procedure category
    #[serde(rename = "MV")]
    MultiLevelProcedureCategory,
    /// Chemical Abstracts Service Type 1 name of a chemical
    #[serde(rename = "N1")]
    ChemicalAbstractsServiceType1,
    /// Name aliases
    #[serde(rename = "NA")]
    NameAliases,
    /// Name of Supplementary Concept
    #[serde(rename = "NM")]
    SupplementaryConceptName,
    /// HL7 non-preferred for language term
    #[serde(rename = "NPT")]
    Hl7NonPreferredLanguageTerm,
    /// Non-preferred term
    #[serde(rename = "NP")]
    NonPreferredTerm,
    /// Short form of non-preferred term
    #[serde(rename = "NS")]
    ShortNonPreferredTerm,
    /// Expanded form of non-preferred term
    #[serde(rename = "NX")]
    ExpandedNonPreferredTerm,
    /// Obsolete active fully specified name
    #[serde(rename = "OAF")]
    ObsoleteActiveFullySpecifiedName,
    /// Obsolete Modifier Abbreviation
    #[serde(rename = "OAM")]
    ObsoleteModifierAbbreviation,
    /// Obsolete active preferred term
    #[serde(rename = "OAP")]
    ObsoleteActivePreferredTerm,
    /// Obsolete active synonym
    #[serde(rename = "OAS")]
    ObsoleteActiveSynonym,
    /// Obsolete abbreviation
    #[serde(rename = "OA")]
    ObsoleteAbbreviation,
    /// Nursing outcomes
    #[serde(rename = "OC")]
    NursingOutcomes,
    /// Obsolete Display Name
    #[serde(rename = "ODN")]
    ObsoleteDisplayName,
    /// Obsolete entry term
    #[serde(rename = "OET")]
    ObsoleteEntryTerm,
    /// Obsolete fully specified name
    #[serde(rename = "OF")]
    ObsoleteFullySpecifiedName,
    /// Obsolete Long common name
    #[serde(rename = "OLC")]
    ObsoleteLongCommonName,
    /// Obsolete LOINC group name
    #[serde(rename = "OLG")]
    ObsoleteLoincGroupName,
    /// Japanese Non-current Lower Level Term (kana1)
    #[serde(rename = "OLJKN1")]
    JapaneseNonCurrentLowerLevelTermKana1,
    /// Japanese Non-current Lower Level Term (kana)
    #[serde(rename = "OLJKN")]
    JapaneseNonCurrentLowerLevelTermKana,
    /// Non-current Lower Level Term
    #[serde(rename = "OL")]
    NonCurrentLowerLevelTerm,
    /// Obsolete modifiers in HCPCS
    #[serde(rename = "OM")]
    ObsoleteModifiersHcpcs,
    /// Obsolete non-preferred for language term
    #[serde(rename = "ONP")]
    ObsoleteNonPreferredLanguageTerm,
    /// Obsolete official short name
    #[serde(rename = "OOSN")]
    ObsoleteOfficialShortName,
    /// Obsolete preferred term, natural language form
    #[serde(rename = "OPN")]
    ObsoletePreferredTermNatural,
    /// Obsolete preferred name
    #[serde(rename = "OP")]
    ObsoletePreferredName,
    /// Orders
    #[serde(rename = "OR")]
    Orders,
    /// Japanese System-organ class (WHO Adverse Reaction Terminology) (kana1)
    #[serde(rename = "OSJKN1")]
    JapaneseSystemOrganClassKana1,
    /// Japanese System-organ class (WHO Adverse Reaction Terminology) (kana)
    #[serde(rename = "OSJKN")]
    JapaneseSystemOrganClassKana,
    /// Official short name
    #[serde(rename = "OSN")]
    OfficialShortName,
    /// System-organ class
    #[serde(rename = "OS")]
    SystemOrganClass,
    /// Preferred entry term for Supplementary Concept
    #[serde(rename = "PCE")]
    PreferredEntryTermSupplementaryConcept,
    /// Preferred "trimmed term" in ICPC
    #[serde(rename = "PC")]
    PreferredTrimmedTermIcpc,
    /// Preferred entry term
    #[serde(rename = "PEP")]
    PreferredEntryTerm,
    /// Phenotype entry term
    #[serde(rename = "PHENO_ET")]
    PhenotypeEntryTerm,
    /// Phenotype
    #[serde(rename = "PHENO")]
    Phenotype,
    /// Name from a precise ingredient
    #[serde(rename = "PIN")]
    PreciseIngredientName,
    /// Machine permutation
    #[serde(rename = "PM")]
    MachinePermutation,
    /// Metathesaurus preferred name
    #[serde(rename = "PN")]
    MetathesaurusPreferredName,
    /// Place of service
    #[serde(rename = "POS")]
    PlaceOfService,
    /// Qualifier for a problem
    #[serde(rename = "PQ")]
    ProblemQualifier,
    /// Name of a problem
    #[serde(rename = "PR")]
    ProblemName,
    /// Protocol selection criteria
    #[serde(rename = "PSC")]
    ProtocolSelectionCriteria,
    /// Prescribable Names
    #[serde(rename = "PSN")]
    PrescribableNames,
    /// Short forms that needed full specification
    #[serde(rename = "PS")]
    ShortFormsFullSpecification,
    /// Preferred Allelic Variant
    #[serde(rename = "PTAV")]
    PreferredAllelicVariant,
    /// Preferred Clinical Synopsis
    #[serde(rename = "PTCS")]
    PreferredClinicalSynopsis,
    /// British preferred term
    #[serde(rename = "PTGB")]
    BritishPreferredTerm,
    /// Japanese Designated preferred name (kana1)
    #[serde(rename = "PTJKN1")]
    JapaneseDesignatedPreferredNameKana1,
    /// Japanese Designated preferred name (kana)
    #[serde(rename = "PTJKN")]
    JapaneseDesignatedPreferredNameKana,
    /// Preferred term, natural language form
    #[serde(rename = "PTN")]
    PreferredTermNatural,
    /// Designated preferred name
    #[serde(rename = "PT")]
    DesignatedPreferredName,
    /// Preferred qualifier term
    #[serde(rename = "PXQ")]
    PreferredQualifierTerm,
    /// Expanded preferred terms (pair with PS)
    #[serde(rename = "PX")]
    ExpandedPreferredTerms,
    /// Qualifier abbreviation
    #[serde(rename = "QAB")]
    QualifierAbbreviation,
    /// Qualifier entry version
    #[serde(rename = "QEV")]
    QualifierEntryVersion,
    /// Root abbreviation
    #[serde(rename = "RAB")]
    RootAbbreviation,
    /// Root hierarchical term
    #[serde(rename = "RHT")]
    RootHierarchicalTerm,
    /// Root preferred term
    #[serde(rename = "RPT")]
    RootPreferredTerm,
    /// Root synonym
    #[serde(rename = "RSY")]
    RootSynonym,
    /// Extracted related names in SNOMED2
    #[serde(rename = "RS")]
    ExtractedRelatedNamesSnomed2,
    /// Term related to, but often considered non-synonymous with, preferred term
    #[serde(rename = "RT")]
    RelatedTerm,
    /// Rxnorm Preferred Ingredient
    #[serde(rename = "RXN_IN")]
    RxnormPreferredIngredient,
    /// Rxnorm Preferred
    #[serde(rename = "RXN_PT")]
    RxnormPreferred,
    /// Semantic Branded Drug Component
    #[serde(rename = "SBDC")]
    SemanticBrandedDrugComponent,
    /// Semantic branded drug and form with precise ingredient
    #[serde(rename = "SBDFP")]
    SemanticBrandedDrugFormPrecise,
    /// Semantic branded drug and form
    #[serde(rename = "SBDF")]
    SemanticBrandedDrugForm,
    /// Semantic branded drug group
    #[serde(rename = "SBDG")]
    SemanticBrandedDrugGroup,
    /// Semantic branded drug
    #[serde(rename = "SBD")]
    SemanticBrandedDrug,
    /// Named subset of a source
    #[serde(rename = "SB")]
    NamedSubsetSource,
    /// Scale
    #[serde(rename = "SCALE")]
    Scale,
    /// Semantic Drug Component
    #[serde(rename = "SCDC")]
    SemanticDrugComponent,
    /// Semantic clinical drug and form with precise ingredient
    #[serde(rename = "SCDFP")]
    SemanticClinicalDrugFormPrecise,
    /// Semantic clinical drug and form
    #[serde(rename = "SCDF")]
    SemanticClinicalDrugForm,
    /// Semantic clinical drug group with precise ingredient
    #[serde(rename = "SCDGP")]
    SemanticClinicalDrugGroupPrecise,
    /// Semantic clinical drug group
    #[serde(rename = "SCDG")]
    SemanticClinicalDrugGroup,
    /// Semantic Clinical Drug
    #[serde(rename = "SCD")]
    SemanticClinicalDrug,
    /// Scientific name
    #[serde(rename = "SCN")]
    ScientificName,
    /// Special Category term
    #[serde(rename = "SC")]
    SpecialCategoryTerm,
    /// CCS diagnosis categories
    #[serde(rename = "SD")]
    CcsDiagnosisCategories,
    /// Name of a sign or symptom of a problem
    #[serde(rename = "SI")]
    SignSymptomName,
    /// Standardised MedDRA Query
    #[serde(rename = "SMQ")]
    StandardisedMeddraQuery,
    /// CCS procedure categories
    #[serde(rename = "SP")]
    CcsProcedureCategories,
    /// Source short name (UMLS Knowledge Source Server)
    #[serde(rename = "SSN")]
    SourceShortName,
    /// Synonymous "short" forms
    #[serde(rename = "SS")]
    SynonymousShortForms,
    /// Step
    #[serde(rename = "ST")]
    Step,
    /// Active Substance
    #[serde(rename = "SU")]
    ActiveSubstance,
    /// Mixed-case component synonym with expanded abbreviations
    #[serde(rename = "SX")]
    MixedCaseComponentSynonym,
    /// British synonym
    #[serde(rename = "SYGB")]
    BritishSynonym,
    /// Designated alias
    #[serde(rename = "SYN")]
    DesignatedAlias,
    /// Designated synonym
    #[serde(rename = "SY")]
    DesignatedSynonym,
    /// Task
    #[serde(rename = "TA")]
    Task,
    /// Term class
    #[serde(rename = "TC")]
    TermClass,
    /// Name of the target of an intervention
    #[serde(rename = "TG")]
    InterventionTargetName,
    /// Tall Man synonym
    #[serde(rename = "TMSY")]
    TallManSynonym,
    /// Topical qualifier
    #[serde(rename = "TQ")]
    TopicalQualifier,
    /// CCPSS synthesized problems for TC termgroup
    #[serde(rename = "TX")]
    CcppsSynthesizedProblems,
    /// Unique common name
    #[serde(rename = "UCN")]
    UniqueCommonName,
    /// Unique equivalent name
    #[serde(rename = "UE")]
    UniqueEquivalentName,
    /// Unique scientific name
    #[serde(rename = "USN")]
    UniqueScientificName,
    /// Unique synonym
    #[serde(rename = "USY")]
    UniqueSynonym,
    /// Versioned abbreviation
    #[serde(rename = "VAB")]
    VersionedAbbreviation,
    /// Versioned preferred term
    #[serde(rename = "VPT")]
    VersionedPreferredTerm,
    /// Versioned synonym
    #[serde(rename = "VSY")]
    VersionedSynonym,
    /// Value Set
    #[serde(rename = "VS")]
    ValueSet,
    /// Expanded descriptor in AOD
    #[serde(rename = "XD")]
    ExpandedDescriptorAod,
    /// Cross mapping set
    #[serde(rename = "XM")]
    CrossMappingSet,
    /// Alternate name for a qualifier
    #[serde(rename = "XQ")]
    AlternateQualifierName,
}

/// Source Restriction Level (SRL)
///
/// Defines the level of restriction applied to a source,
/// ranging from general terms to specific license appendices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SourceRestrictionLevel {
    /// No additional restrictions; general terms of the license agreement
    /// apply.
    #[serde(rename = "0")]
    Level0 = 0,
    /// General terms + additional restrictions in category 12.1
    #[serde(rename = "1")]
    Level1 = 1,
    /// General terms + additional restrictions in category 12.2
    #[serde(rename = "2")]
    Level2 = 2,
    /// General terms + additional restrictions in category 12.3
    #[serde(rename = "3")]
    Level3 = 3,
    /// General terms + additional restrictions in category 12.4
    #[serde(rename = "4")]
    Level4 = 4,
    /// General terms + SNOMED CT Affiliate License in Appendix 2
    #[serde(rename = "9")]
    Level9 = 9,
}

/// Suppressibility Flag (SUPPRESS)
///
/// Indicates whether a term is suppressible and the reason for that status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuppressStatus {
    /// Suppressible due to editor decision
    #[serde(rename = "E")]
    EditorDecision,

    /// Not suppressible
    #[serde(rename = "N")]
    NotSuppressible,

    /// Obsolete, SAB, TTY may be independently suppressible
    #[serde(rename = "O")]
    Obsolete,

    /// Suppressible due to SAB, TTY
    #[serde(rename = "Y")]
    Suppressible,

    /// Suppressibility not yet assigned by the UMLS
    #[serde(rename = "")]
    Unassigned,
}

/// Custom deserializer to convert UMLS 'Y'/'N' flags into a boolean.
///
/// # Arguments
/// * `deserializer` - The Serde deserializer instance.
///
/// # Returns
/// * `Ok(true)` if the input character is 'Y'.
/// * `Ok(false)` for any other character (typically 'N').
///
/// # Errors
/// * `Err` if the input cannot be deserialized as a character.
pub fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(char::deserialize(deserializer)? == 'Y')
}

/// Custom deserializer to convert UMLS 'Y'/'N' flags into a boolean.
///
/// # Arguments
/// * `deserializer` - The Serde deserializer instance.
///
/// # Returns
/// * `Ok(Some(true))` if the input character is 'Y'.
/// * `Ok(Some(false))` for any other character (typically 'N').
/// * `Ok(None)` if it isn't indicated.
///
/// # Errors
/// * `Err` if the input cannot be deserialized as a optional character.
pub fn deserialize_opt_yes_no<'de, D>(
    deserializer: D,
) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<char>::deserialize(deserializer)?.map(|c| c == 'Y'))
}
