use std::{cell::RefCell, rc::Rc};

use oxc_ast::{ast::SourceFile, AstKind};

use crate::opt_rc_cell;

use super::types::{CompilerOptions, FlowFlags, FlowLabel, FlowNode, FlowUnreachable, ScriptTarget, Symbol, __String};

// #region: 332
pub struct ActiveLabel<'a> {
    pub next: opt_rc_cell!(ActiveLabel<'a>),
    pub name: __String,
    pub breakTarget: FlowLabel<'a>,
    pub continueTarget: opt_rc_cell!(FlowLabel<'a>),
    pub referenced: bool,
}
// #endregion: 338

// #region: 519
pub struct Binder<'a> {
    pub file: Option<SourceFile<'a>>,
    pub options: Option<CompilerOptions>,
    pub languageVersion: Option<ScriptTarget>,
    pub parent: Option<AstKind<'a>>,
    // pub container: Option<IsContainerOrEntityNameExpression>,
    // pub thisParentContainer: Option<IsContainerOrEntityNameExpression>, // Container one level up
    // pub blockScopeContainer: Option<IsBlockScopedContainer>,
    // pub lastContainer: Option<HasLocals>,
    // pub delayedTypeAliases: Vec<JsDocTypeOrCallbackOrEnumTag>,
    pub seenThisKeyword: bool,
    // pub jsDocImports: Vec<JSDocImportTag>,

    // state used by control flow analysis
    pub currentFlow: opt_rc_cell!(FlowNode<'a>),
    pub currentBreakTarget: opt_rc_cell!(FlowLabel<'a>),
    pub currentContinueTarget: opt_rc_cell!(FlowLabel<'a>),
    pub currentReturnTarget: opt_rc_cell!(FlowLabel<'a>),
    pub currentTrueTarget: opt_rc_cell!(FlowLabel<'a>),
    pub currentFalseTarget: opt_rc_cell!(FlowLabel<'a>),
    pub currentExceptionTarget: opt_rc_cell!(FlowLabel<'a>),
    pub preSwitchCaseFlow: opt_rc_cell!(FlowNode<'a>),
    pub activeLabelList: Option<ActiveLabel<'a>>,
    pub hasExplicitReturn: bool,
    pub hasFlowEffects: bool,

    // state used for emit helpers
    // pub emitFlags: NodeFlags,

    // If this file is an external module, then it is automatically in strict-mode according to
    // ES6.  If it is not an external module, then we'll determine if it is in strict mode or
    // not depending on if we see "use strict" in certain places or if we hit a class/namespace
    // or if compiler options contain alwaysStrict.
    pub inStrictMode: bool,

    // If we are binding an assignment pattern, we will bind certain expressions differently.
    pub inAssignmentPattern: bool,

    pub symbolCount: usize,

    pub symbol: Option<Symbol<'a>>,
    pub classifiableNames: Option<std::collections::HashSet<__String>>,

    pub unreachableFlow: FlowNode<'a>,
    pub reportedUnreachableFlow: FlowNode<'a>,
    // pub bindBinaryExpressionFlow: FlowNode<'a>,
}

impl<'a> Default for Binder<'a> {
    fn default() -> Self {
        Self {
            file: None,
            options: None,
            languageVersion: None,
            parent: None,
            // container
            // thisParentContainer
            // blockScopeContainer
            // lastContainer
            // delayedTypeAliases
            seenThisKeyword: false,
            // jsDocImports
            currentFlow: None,
            currentBreakTarget: None,
            currentContinueTarget: None,
            currentReturnTarget: None,
            currentTrueTarget: None,
            currentFalseTarget: None,
            currentExceptionTarget: None,
            preSwitchCaseFlow: None,
            activeLabelList: None,
            hasExplicitReturn: false,
            hasFlowEffects: false,
            // emitFlags
            inStrictMode: false,
            inAssignmentPattern: false,
            symbolCount: 0,
            symbol: None,
            classifiableNames: None,
            unreachableFlow: FlowNode::Unreachable(FlowUnreachable { flags: FlowFlags(0), id: 0 }),
            reportedUnreachableFlow: FlowNode::Unreachable(FlowUnreachable { flags: FlowFlags(0), id: 0 }),
            // bind_binary_expression_flow: createBindBinaryExpressionFlow(),
        }
    }
}
// #endregion: 570
