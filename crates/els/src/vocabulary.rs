//! The engineering vocabulary: the names ELS protocols use for their terms.
//!
//! Every term `software.change/1` and `incident.response/1` names is declared here once, with its
//! category and its marking. This module holds names and what they mean, nothing more: what a
//! claim, a piece of evidence or an obligation *is*, and how one is decided, is Canon's.
//!
//! Spelling: claim and action ids are dotted, `<subject>.<predicate>`; artifact, evidence,
//! obligation and outcome ids are single snake_case tokens.

use std::fmt;

use b10x_canon::{ActionId, ClaimId};

/// The kind of name a term is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    ArtifactKind,
    EvidenceKind,
    ClaimId,
    ActionId,
    ObligationId,
    OutcomeId,
}

/// Where a term belongs: every engineering domain, or only one protocol's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Marking {
    /// Not specific to Git, pull requests or code.
    Core,
    /// Only makes sense for Git, pull requests or code.
    SoftwareChange,
}

impl Marking {
    pub const fn as_str(self) -> &'static str {
        match self {
            Marking::Core => "core",
            Marking::SoftwareChange => "software.change",
        }
    }
}

impl fmt::Display for Marking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One vocabulary entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Term {
    pub id: &'static str,
    pub category: Category,
    pub marking: Marking,
    pub meaning: &'static str,
}

impl Term {
    /// The Canon claim id this term names, if it is a claim id.
    pub fn claim_id(&self) -> Option<ClaimId> {
        (self.category == Category::ClaimId).then(|| ClaimId(self.id.to_owned()))
    }

    /// The Canon action id this term names, if it is an action id.
    pub fn action_id(&self) -> Option<ActionId> {
        (self.category == Category::ActionId).then(|| ActionId(self.id.to_owned()))
    }
}

/// A lookup of a name no vocabulary entry declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownTerm {
    pub term: String,
}

impl fmt::Display for UnknownTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` is not an ELS engineering term", self.term)
    }
}

impl std::error::Error for UnknownTerm {}

/// Every declared term.
pub fn terms() -> &'static [Term] {
    TERMS
}

/// The entry `id` names, or a refusal naming `id`.
pub fn lookup(id: &str) -> Result<&'static Term, UnknownTerm> {
    TERMS
        .iter()
        .find(|t| t.id == id)
        .ok_or_else(|| UnknownTerm {
            term: id.to_owned(),
        })
}

const fn term(
    id: &'static str,
    category: Category,
    marking: Marking,
    meaning: &'static str,
) -> Term {
    Term {
        id,
        category,
        marking,
        meaning,
    }
}

use Category::{ActionId as Action, ArtifactKind as Artifact, ClaimId as Claim};
use Category::{EvidenceKind as Evidence, ObligationId as Obligation, OutcomeId as Outcome};
use Marking::{Core, SoftwareChange};

const TERMS: &[Term] = &[
    // artifact kinds
    term(
        "intent",
        Artifact,
        Core,
        "what the change or response is meant to achieve",
    ),
    term(
        "system_specification",
        Artifact,
        Core,
        "the specified behaviour of the system concerned",
    ),
    term(
        "plan",
        Artifact,
        Core,
        "the intended steps toward the intent",
    ),
    term(
        "release",
        Artifact,
        Core,
        "a versioned, deployable unit of the system",
    ),
    term(
        "deployment",
        Artifact,
        Core,
        "a release running in an environment",
    ),
    term(
        "service",
        Artifact,
        Core,
        "the running service a case concerns",
    ),
    term(
        "implementation",
        Artifact,
        SoftwareChange,
        "a revision of the code that realizes the plan",
    ),
    // evidence kinds
    term(
        "operational_observation",
        Evidence,
        Core,
        "an observation of the running system",
    ),
    term(
        "objective_observation",
        Evidence,
        Core,
        "an observation of whether the intent's objective is met",
    ),
    term(
        "impact_assessment",
        Evidence,
        Core,
        "an assessment of who and what an incident affects",
    ),
    term(
        "cause_analysis",
        Evidence,
        Core,
        "an analysis of why an incident happened",
    ),
    term(
        "test_result",
        Evidence,
        SoftwareChange,
        "the result of running tests against an implementation revision",
    ),
    term(
        "code_review",
        Evidence,
        SoftwareChange,
        "a review of an implementation revision",
    ),
    term(
        "build_provenance",
        Evidence,
        SoftwareChange,
        "the record of how a release was built from an implementation",
    ),
    // claim ids
    term(
        "release.proven",
        Claim,
        Core,
        "the release comes from a verified implementation and carries current build provenance",
    ),
    term(
        "deployment.healthy",
        Claim,
        Core,
        "the deployment is running healthily",
    ),
    term(
        "objective.realized",
        Claim,
        Core,
        "the intent's objective is met",
    ),
    term(
        "impact.bounded",
        Claim,
        Core,
        "the incident's impact is known and contained",
    ),
    term(
        "service.healthy",
        Claim,
        Core,
        "the service is running healthily",
    ),
    term(
        "cause.identified",
        Claim,
        Core,
        "the incident's cause is known",
    ),
    term(
        "tests.pass",
        Claim,
        SoftwareChange,
        "the tests pass for the current implementation revision",
    ),
    term(
        "implementation.reviewed",
        Claim,
        SoftwareChange,
        "the current implementation revision is reviewed",
    ),
    term(
        "implementation.verified",
        Claim,
        SoftwareChange,
        "the current implementation revision is verified",
    ),
    // action ids
    term("metrics.inspect", Action, Core, "read the system's metrics"),
    term("logs.search", Action, Core, "search the system's logs"),
    term(
        "release.inspect",
        Action,
        Core,
        "read what a release contains",
    ),
    term(
        "release.rollback",
        Action,
        Core,
        "return to a previous release",
    ),
    term(
        "traffic.shift",
        Action,
        Core,
        "move traffic between deployments",
    ),
    term("emergency.leave", Action, Core, "leave emergency mode"),
    term(
        "repository.inspect",
        Action,
        SoftwareChange,
        "read the repository",
    ),
    term(
        "repository.edit",
        Action,
        SoftwareChange,
        "change the repository's working revision",
    ),
    term(
        "repository.merge",
        Action,
        SoftwareChange,
        "merge a revision into the repository's mainline",
    ),
    term(
        "tests.run",
        Action,
        SoftwareChange,
        "run the tests against an implementation revision",
    ),
    // obligation ids
    term(
        "restore_service",
        Obligation,
        Core,
        "bring the affected service back to health",
    ),
    // outcome ids
    term(
        "accepted",
        Outcome,
        Core,
        "the case closed with its result accepted",
    ),
];
