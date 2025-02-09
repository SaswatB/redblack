use std::{cell::RefCell, collections::HashSet, rc::Rc};

use oxc_ast::{ast::SourceFile, AstKind, GetChildren};

use crate::{define_flags, flag_names_impl, opt_rc_cell};

use super::{
    factory::node_tests::isClassStaticBlockDeclaration,
    rb_extra::{AstKindExt, SourceFileExt},
    rb_unions::IsContainerOrEntityNameExpression,
    types::{CompilerOptions, FlowFlags, FlowLabel, FlowNode, FlowUnreachable, HasLocals, IsBlockScopedContainer, ScriptTarget, Symbol, __String},
    utilities::{getEmitScriptTarget, isObjectLiteralOrClassExpressionMethodOrAccessor, isPartOfTypeQuery},
    utilitiesPublic::isFunctionLike,
};

// region: 332
pub struct ActiveLabel<'a> {
    pub next: opt_rc_cell!(ActiveLabel<'a>),
    pub name: __String,
    pub breakTarget: FlowLabel<'a>,
    pub continueTarget: opt_rc_cell!(FlowLabel<'a>),
    pub referenced: bool,
}
// endregion: 338

// region: 475
// @internal
define_flags!(ContainerFlags {
    // The current node is not a container, and no container manipulation should happen before
    // recursing into it.
    None = 0,

    // The current node is a container.  It should be set as the current container (and block-
    // container) before recursing into it.  The current node does not have locals.  Examples:
    //
    //      Classes, ObjectLiterals, TypeLiterals, Interfaces...
    IsContainer = 1 << 0,

    // The current node is a block-scoped-container.  It should be set as the current block-
    // container before recursing into it.  Examples:
    //
    //      Blocks (when not parented by functions), Catch clauses, For/For-in/For-of statements...
    IsBlockScopedContainer = 1 << 1,

    // The current node is the container of a control flow path. The current control flow should
    // be saved and restored, and a new control flow initialized within the container.
    IsControlFlowContainer = 1 << 2,

    IsFunctionLike = 1 << 3,
    IsFunctionExpression = 1 << 4,
    HasLocals = 1 << 5,
    IsInterface = 1 << 6,
    IsObjectLiteralOrClassExpressionMethodOrAccessor = 1 << 7,
});
// endregion: 502

// region: 509 - disjoint indentation
pub struct Binder<'a> {
    pub file: Option<&'a SourceFile<'a>>,
    pub options: Option<&'a CompilerOptions>,
    pub languageVersion: Option<ScriptTarget>,
    pub parent: Option<AstKind<'a>>,
    pub container: Option<IsContainerOrEntityNameExpression<'a>>,
    pub thisParentContainer: Option<IsContainerOrEntityNameExpression<'a>>, // Container one level up
    pub blockScopeContainer: Option<IsBlockScopedContainer<'a>>,
    pub lastContainer: Option<HasLocals<'a>>,
    // pub delayedTypeAliases: Option<Vec<JsDocTypeOrCallbackOrEnumTag>>,
    pub seenThisKeyword: bool,
    // pub jsDocImports: Option<Vec<JSDocImportTag>>,

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
    pub classifiableNames: opt_rc_cell!(HashSet<__String>),

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
            container: None,
            thisParentContainer: None,
            blockScopeContainer: None,
            lastContainer: None,
            // delayedTypeAliases: None,
            seenThisKeyword: false,
            // jsDocImports: None,
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
            // emitFlags: NodeFlags::None,
            inStrictMode: false,
            inAssignmentPattern: false,
            symbolCount: 0,
            symbol: None,
            classifiableNames: None,
            unreachableFlow: FlowNode::Unreachable(FlowUnreachable { flags: FlowFlags(0), id: 0 }),
            reportedUnreachableFlow: FlowNode::Unreachable(FlowUnreachable { flags: FlowFlags(0), id: 0 }),
            // bindBinaryExpressionFlow: createBindBinaryExpressionFlow(),
        }
    }
}

impl<'a> Binder<'a> {
    fn bindSourceFile(&mut self, f: &'a SourceFile<'a>, opts: &'a CompilerOptions) {
        self.file = Some(f);
        self.options = Some(opts);
        self.languageVersion = Some(getEmitScriptTarget(opts));
        self.inStrictMode = true; // todo(RB): bindInStrictMode(&self.file.as_ref().unwrap(), &opts);
        self.classifiableNames = Some(Rc::new(RefCell::new(HashSet::new())));
        self.symbolCount = 0;

        // Symbol = objectAllocator.getSymbolConstructor();

        // Attach debugging information if necessary
        // Debug::attachFlowNodeDebugInfo(&self.unreachableFlow);
        // Debug::attachFlowNodeDebugInfo(&self.reportedUnreachableFlow);

        if f.locals().is_none() {
            // tracing.push(tracing::Phase::Bind, "bindSourceFile", Some(json!({ "path": self.file.as_ref().unwrap().path })), /*separateBeginAndEnd*/ true);
            self.bind(Some(f.to_ast_kind()));
            // tracing.pop();
            f.set_symbolCount(self.symbolCount);
            f.set_classifiableNames(self.classifiableNames.clone());
            // self.delayedBindJSDocTypedefTag();
            // self.bindJSDocImports();
        }

        self.file = None;
        self.options = None;
        self.languageVersion = None;
        self.parent = None;
        self.container = None;
        self.thisParentContainer = None;
        self.blockScopeContainer = None;
        self.lastContainer = None;
        // self.delayedTypeAliases = None;
        // self.jsDocImports = None;
        self.seenThisKeyword = false;
        self.currentFlow = None;
        self.currentBreakTarget = None;
        self.currentContinueTarget = None;
        self.currentReturnTarget = None;
        self.currentTrueTarget = None;
        self.currentFalseTarget = None;
        self.currentExceptionTarget = None;
        self.activeLabelList = None;
        self.hasExplicitReturn = false;
        self.hasFlowEffects = false;
        self.inAssignmentPattern = false;
        // self.emitFlags = NodeFlags::None;
    }

    // endregion: 628

    // region: 960
    // All container nodes are kept on a linked list in declaration order. This list is used by
    // the getLocalNameOfContainer function in the type checker to validate that the local name
    // used for a container is unique.
    fn bindContainer(&mut self, node: &AstKind<'a>, container_flags: ContainerFlags) {
        // todo(RB): continue conversion from here
        // // Before we recurse into a node's children, we first save the existing parent, container
        // // and block-container.  Then after we pop out of processing the children, we restore
        // // these saved values.
        // let save_container = self.container.clone();
        // let save_this_parent_container = self.thisParentContainer.clone();
        // let saved_block_scope_container = self.blockScopeContainer.clone();

        // // Depending on what kind of node this is, we may have to adjust the current container
        // // and block-container.   If the current node is a container, then it is automatically
        // // considered the current block-container as well.  Also, for containers that we know
        // // may contain locals, we eagerly initialize the .locals field. We do this because
        // // it's highly likely that the .locals will be needed to place some child in (for example,
        // // a parameter, or variable declaration).
        // //
        // // However, we do not proactively create the .locals for block-containers because it's
        // // totally normal and common for block-containers to never actually have a block-scoped
        // // variable in them.  We don't want to end up allocating an object for every 'block' we
        // // run into when most of them won't be necessary.
        // //
        // // Finally, if this is a block-container, then we clear out any existing .locals object
        // // it may contain within it.  This happens in incremental scenarios.  Because we can be
        // // reusing a node from a previous compilation, that node may have had 'locals' created
        // // for it.  We must clear this so we don't accidentally move any stale data forward from
        // // a previous compilation.
        // if container_flags.contains(ContainerFlags::IsContainer) {
        //     if !matches!(node, AstKind::ArrowFunction(_)) {
        //         self.thisParentContainer = self.container.clone();
        //     }
        //     self.container = Some(node.clone());
        //     self.blockScopeContainer = Some(node.clone());
        //     if container_flags.contains(ContainerFlags::HasLocals) {
        //         // node.locals = createSymbolTable();
        //         // addToContainerChain(node);
        //     }
        // } else if container_flags.contains(ContainerFlags::IsBlockScopedContainer) {
        //     self.blockScopeContainer = Some(node.clone());
        //     if container_flags.contains(ContainerFlags::HasLocals) {
        //         // node.locals = None;
        //     }
        // }

        // if container_flags.contains(ContainerFlags::IsControlFlowContainer) {
        //     let save_current_flow = self.currentFlow.clone();
        //     let save_break_target = self.currentBreakTarget.clone();
        //     let save_continue_target = self.currentContinueTarget.clone();
        //     let save_return_target = self.currentReturnTarget.clone();
        //     let save_exception_target = self.currentExceptionTarget.clone();
        //     let save_active_label_list = self.activeLabelList.clone();
        //     let save_has_explicit_return = self.hasExplicitReturn;

        //     let is_immediately_invoked = (container_flags.contains(ContainerFlags::IsFunctionExpression) &&
        //         // !hasSyntacticModifier(node, ModifierFlags::Async) &&
        //         // !(node as FunctionLikeDeclaration).asteriskToken &&
        //         // !!getImmediatelyInvokedFunctionExpression(node)
        //         false)
        //         || matches!(node, AstKind::ClassStaticBlock(_));

        //     // A non-async, non-generator IIFE is considered part of the containing control flow. Return statements behave
        //     // similarly to break statements that exit to a label just past the statement body.
        //     if !is_immediately_invoked {
        //         self.currentFlow = Some(FlowNode::new(FlowFlags::Start));
        //         if container_flags.intersects(ContainerFlags::IsFunctionExpression | ContainerFlags::IsObjectLiteralOrClassExpressionMethodOrAccessor) {
        //             // currentFlow.node = node;
        //         }
        //     }

        //     // We create a return control flow graph for IIFEs and constructors. For constructors
        //     // we use the return control flow graph in strict property initialization checks.
        //     self.currentReturnTarget = if is_immediately_invoked || matches!(node, AstKind::Constructor(_)) || (self.isInJSFile(node) && (matches!(node, AstKind::FunctionDeclaration(_)) || matches!(node, AstKind::FunctionExpression(_)))) { Some(self.createBranchLabel()) } else { None };

        //     self.currentExceptionTarget = None;
        //     self.currentBreakTarget = None;
        //     self.currentContinueTarget = None;
        //     self.activeLabelList = None;
        //     self.hasExplicitReturn = false;

        //     self.bindChildren(node);

        //     // Reset all reachability check related flags on node (for incremental scenarios)
        //     // node.flags &= !NodeFlags::ReachabilityAndEmitFlags;

        //     if let Some(current_flow) = &self.currentFlow {
        //         if !current_flow.flags.contains(FlowFlags::Unreachable) && container_flags.contains(ContainerFlags::IsFunctionLike) {
        //             // node.flags |= NodeFlags::HasImplicitReturn;
        //             // if self.hasExplicitReturn {
        //             //     node.flags |= NodeFlags::HasExplicitReturn;
        //             // }
        //             // node.endFlowNode = self.currentFlow.clone();
        //         }
        //     }

        //     if matches!(node, AstKind::SourceFile(_)) {
        //         // node.flags |= self.emitFlags;
        //         // node.endFlowNode = self.currentFlow.clone();
        //     }

        //     if let Some(current_return_target) = &self.currentReturnTarget {
        //         self.addAntecedent(current_return_target, self.currentFlow.as_ref().unwrap());
        //         self.currentFlow = Some(self.finishFlowLabel(current_return_target));
        //         if matches!(node, AstKind::Constructor(_)) || matches!(node, AstKind::ClassStaticBlock(_)) || (self.isInJSFile(node) && (matches!(node, AstKind::FunctionDeclaration(_)) || matches!(node, AstKind::FunctionExpression(_)))) {
        //             // node.returnFlowNode = self.currentFlow.clone();
        //         }
        //     }

        //     if !is_immediately_invoked {
        //         self.currentFlow = save_current_flow;
        //     }
        //     self.currentBreakTarget = save_break_target;
        //     self.currentContinueTarget = save_continue_target;
        //     self.currentReturnTarget = save_return_target;
        //     self.currentExceptionTarget = save_exception_target;
        //     self.activeLabelList = save_active_label_list;
        //     self.hasExplicitReturn = save_has_explicit_return;
        // } else if container_flags.contains(ContainerFlags::IsInterface) {
        //     self.seenThisKeyword = false;
        //     self.bindChildren(node);
        //     // Debug.assertNotNode(node, isIdentifier); // ContainsThis cannot overlap with HasExtendedUnicodeEscape on Identifier
        //     // node.flags = if self.seenThisKeyword {
        //     //     node.flags | NodeFlags::ContainsThis
        //     // } else {
        //     //     node.flags & !NodeFlags::ContainsThis
        //     // };
        // } else {
        self.bindChildren(node);
        // }

        // self.container = save_container;
        // self.thisParentContainer = save_this_parent_container;
        // self.blockScopeContainer = saved_block_scope_container;
    }

    // fn bindEachFunctionsFirst(&mut self, nodes: Option<&[AstKind<'a>]>) {
    //     self.bindEach(nodes, |n| if matches!(n, AstKind::FunctionDeclaration(_)) { self.bind(Some(n)) } else { None });
    //     self.bindEach(nodes, |n| if !matches!(n, AstKind::FunctionDeclaration(_)) { self.bind(Some(n)) } else { None });
    // }

    fn bindEach<F>(&mut self, nodes: Option<&[AstKind<'a>]>, bind_function: F)
    where
        F: Fn(&AstKind<'a>) -> Option<()>,
    {
        if nodes.is_none() {
            return;
        }

        for node in nodes.unwrap() {
            bind_function(node);
        }
    }

    fn bindEachChild(&mut self, node: &AstKind<'a>) {
        for child in node.get_children() {
            self.bind(Some(child));
        }
    }

    fn bindChildren(&mut self, node: &AstKind<'a>) {
        let save_in_assignment_pattern = self.inAssignmentPattern;
        // Most nodes aren't valid in an assignment pattern, so we clear the value here
        // and set it before we descend into nodes that could actually be part of an assignment pattern.
        self.inAssignmentPattern = false;
        // todo(RB): continue conversion from here
        // if self.checkUnreachable(node) {
        //     self.bindEachChild(node);
        //     self.bindJSDoc(node);
        //     self.inAssignmentPattern = save_in_assignment_pattern;
        //     return;
        // }
        // if node.is_statement() && (!self.options.as_ref().unwrap().allowUnreachableCode || node.is_return_statement()) {
        //     node.set_flow_node(self.currentFlow.clone());
        // }
        match node {
            //     AstKind::WhileStatement(while_stmt) => {
            //         self.bindWhileStatement(while_stmt);
            //     }
            //     AstKind::DoWhileStatement(do_stmt) => {
            //         self.bindDoStatement(do_stmt);
            //     }
            //     AstKind::ForStatement(for_stmt) => {
            //         self.bindForStatement(for_stmt);
            //     }
            //     AstKind::ForInStatement(for_in) | AstKind::ForOfStatement(for_of) => {
            //         self.bindForInOrForOfStatement(node);
            //     }
            //     AstKind::IfStatement(if_stmt) => {
            //         self.bindIfStatement(if_stmt);
            //     }
            //     AstKind::ReturnStatement(_) | AstKind::ThrowStatement(_) => {
            //         self.bindReturnOrThrow(node);
            //     }
            //     AstKind::BreakStatement(_) | AstKind::ContinueStatement(_) => {
            //         self.bindBreakOrContinueStatement(node);
            //     }
            //     AstKind::TryStatement(try_stmt) => {
            //         self.bindTryStatement(try_stmt);
            //     }
            //     AstKind::SwitchStatement(switch_stmt) => {
            //         self.bindSwitchStatement(switch_stmt);
            //     }
            //     AstKind::BlockStatement(block) => {
            //         self.bindCaseBlock(block);
            //     }
            //     AstKind::SwitchCase(case) => {
            //         self.bindCaseClause(case);
            //     }
            //     AstKind::ExpressionStatement(expr_stmt) => {
            //         self.bindExpressionStatement(expr_stmt);
            //     }
            //     AstKind::LabeledStatement(labeled) => {
            //         self.bindLabeledStatement(labeled);
            //     }
            //     AstKind::UnaryExpression(unary) if unary.prefix => {
            //         self.bindPrefixUnaryExpressionFlow(unary);
            //     }
            //     AstKind::UnaryExpression(unary) if !unary.prefix => {
            //         self.bindPostfixUnaryExpressionFlow(unary);
            //     }
            //     AstKind::BinaryExpression(binary) => {
            //         if self.isDestructuringAssignment(binary) {
            //             // Carry over whether we are in an assignment pattern to
            //             // binary expressions that could actually be an initializer
            //             self.inAssignmentPattern = save_in_assignment_pattern;
            //             self.bindDestructuringAssignmentFlow(binary);
            //             return;
            //         }
            //         self.bindBinaryExpressionFlow(binary);
            //     }
            //     AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::Delete => {
            //         self.bindDeleteExpressionFlow(unary);
            //     }
            //     AstKind::ConditionalExpression(cond) => {
            //         self.bindConditionalExpressionFlow(cond);
            //     }
            //     AstKind::VariableDeclarator(var_decl) => {
            //         self.bindVariableDeclarationFlow(var_decl);
            //     }
            //     AstKind::MemberExpression(_) | AstKind::OptionalMemberExpression(_) => {
            //         self.bindAccessExpressionFlow(node);
            //     }
            //     AstKind::CallExpression(call) => {
            //         self.bindCallExpressionFlow(call);
            //     }
            //     AstKind::TSNonNullExpression(non_null) => {
            //         self.bindNonNullExpressionFlow(non_null);
            //     }
            //     AstKind::TSTypeAliasDeclaration(_) | AstKind::TSCallSignatureDeclaration(_) | AstKind::TSEnumDeclaration(_) => {
            //         self.bindJSDocTypeAlias(node);
            //     }
            //     AstKind::TSImportType(import) => {
            //         self.bindJSDocImportTag(import);
            //     }
            //     // In source files and blocks, bind functions first to match hoisting that occurs at runtime
            //     AstKind::SourceFile(source_file) => {
            //         self.bindEachFunctionsFirst(&source_file.body);
            //         self.bind(Some(source_file.to_ast_kind()));
            //     }
            //     AstKind::BlockStatement(_) | AstKind::TSModuleBlock(_) => {
            //         self.bindEachFunctionsFirst(node.get_statements());
            //     }
            //     AstKind::ArrayPattern(binding) => {
            //         self.bindBindingElementFlow(binding);
            //     }
            //     AstKind::Parameter(param) => {
            //         self.bindParameterFlow(param);
            //     }
            //     AstKind::ObjectExpression(_) | AstKind::ArrayExpression(_) | AstKind::Property(_) | AstKind::SpreadElement(_) => {
            //         // Carry over whether we are in an assignment pattern of Object and Array literals
            //         // as well as their children that are valid assignment targets.
            //         self.inAssignmentPattern = save_in_assignment_pattern;
            //         self.bindEachChild(node);
            //     }
            _ => {
                self.bindEachChild(node);
            }
        }
        // self.bindJSDoc(node);
        self.inAssignmentPattern = save_in_assignment_pattern;
    }
    // endregion: 1226

    // region: 2741
    fn bind(&mut self, node: Option<AstKind<'a>>) {
        if node.is_none() {
            return;
        }
        let node = node.unwrap();
        // TEST.with(|t| t.borrow_mut().test(self.parent.clone()));
        node.set_parent(self.parent.clone());
        // if tracing { node.tracingPath = self.file.path; }
        let save_in_strict_mode = self.inStrictMode;

        // Even though in the AST the jsdoc @typedef node belongs to the current node,
        // its symbol might be in the same scope with the current node's symbol. Consider:
        //
        //     /** @typedef {string | number} MyType */
        //     function foo();
        //
        // Here the current node is "foo", which is a container, but the scope of "MyType" should
        // not be inside "foo". Therefore we always bind @typedef before bind the parent node,
        // and skip binding this tag later when binding all the other jsdoc tags.

        // First we bind declaration nodes to a symbol if possible. We'll both create a symbol
        // and then potentially add the symbol to an appropriate symbol table. Possible
        // destination symbol tables are:
        //
        //  1) The 'exports' table of the current container's symbol.
        //  2) The 'members' table of the current container's symbol.
        //  3) The 'locals' table of the current container.
        //
        // However, not all symbols will end up in any of these tables. 'Anonymous' symbols
        // (like TypeLiterals for example) will not be put in any table.
        self.bindWorker(&node);

        // Then we recurse into the children of the node to bind them as well. For certain
        // symbols we do specialized work when we recurse. For example, we'll keep track of
        // the current 'container' node when it changes. This helps us know which symbol table
        // a local should go into for example. Since terminal nodes are known not to have
        // children, as an optimization we don't process those.
        // if (node.kind > SyntaxKind.LastToken) {
        let save_parent = self.parent.clone();
        self.parent = Some(node.clone());
        let container_flags = getContainerFlags(&node);
        if container_flags == ContainerFlags::None {
            self.bindChildren(&node);
        } else {
            self.bindContainer(&node, container_flags);
        }
        self.parent = save_parent;
        // } else {
        //     if node.kind == SyntaxKind::EndOfFileToken { self.parent = Some(node); }
        //     self.bindJSDoc(&node);
        //     self.parent = save_parent;
        // }
        self.inStrictMode = save_in_strict_mode;
    }
    // endregion: 2794

    // region: 2836
    fn bindWorker(&mut self, node: &AstKind<'a>) {
        match node {
            /* Strict mode checks */
            AstKind::IdentifierReference(_) | //
            AstKind::BindingIdentifier(_) |
                // ! rb skipping jsdoc
                // for typedef type names with namespaces, bind the new jsdoc type symbol here
                // because it requires all containing namespaces to be in effect, namely the
                // current "blockScopeContainer" needs to be set to its immediate namespace parent.
                // if (node.flags & NodeFlags.IdentifierIsInJSDocNamespace) {
                //     let parentNode = node.parent;
                //     while (parentNode && !isJSDocTypeAlias(parentNode)) {
                //         parentNode = parentNode.parent;
                //     }
                //     bindBlockScopedDeclaration(parentNode as Declaration, SymbolFlags.TypeAlias, SymbolFlags.TypeAliasExcludes);
                //     break;
                // }
                // falls through
            AstKind::ThisExpression(_) => {
                // TODO: Why use `isExpression` here? both Identifier and ThisKeyword are expressions.
                // if (currentFlow && (isExpression(node) || parent.kind === SyntaxKind.ShorthandPropertyAssignment)) {
                node.set_flowNode(self.currentFlow.clone());
                // }
                // TODO: a `ThisExpression` is not an Identifier, this cast is unsound
                // return checkContextualIdentifier(node as Identifier); // todo(RB): checkContextualIdentifier()
            }
            AstKind::TSQualifiedName(_) => {
                if self.currentFlow.is_some() && isPartOfTypeQuery(node) {
                    node.set_flowNode(self.currentFlow.clone());
                }
            }
            AstKind::MetaProperty(_)  |
            AstKind::Super(_) => {
                node.set_flowNode(self.currentFlow.clone());
            }
            AstKind::PrivateIdentifier(private) => {
                // return checkPrivateIdentifier(node as PrivateIdentifier);
            }
            // todo(RB): continue conversion from here
            AstKind::PropertyAccessExpression(_) |
            AstKind::ElementAccessExpression(_) => {
                // const expr = node as PropertyAccessExpression | ElementAccessExpression;
                // if (currentFlow && isNarrowableReference(expr)) {
                //     expr.flowNode = currentFlow;
                // }
                // if (isSpecialPropertyDeclaration(expr)) {
                //     bindSpecialPropertyDeclaration(expr);
                // }
                // if (
                //     isInJSFile(expr) &&
                //     file.commonJsModuleIndicator &&
                //     isModuleExportsAccessExpression(expr) &&
                //     !lookupSymbolForName(blockScopeContainer, "module" as __String)
                // ) {
                //     declareSymbol(file.locals!, /*parent*/ undefined, expr.expression, SymbolFlags.FunctionScopedVariable | SymbolFlags.ModuleExports, SymbolFlags.FunctionScopedVariableExcludes);
                // }
            }
            AstKind::BinaryExpression(expr) => {
                // const specialKind = getAssignmentDeclarationKind(node as BinaryExpression);
                // switch (specialKind) {
                //     case AssignmentDeclarationKind.ExportsProperty:
                //         bindExportsPropertyAssignment(node as BindableStaticPropertyAssignmentExpression);
                //         break;
                //     case AssignmentDeclarationKind.ModuleExports:
                //         bindModuleExportsAssignment(node as BindablePropertyAssignmentExpression);
                //         break;
                //     case AssignmentDeclarationKind.PrototypeProperty:
                //         bindPrototypePropertyAssignment((node as BindableStaticPropertyAssignmentExpression).left, node);
                //         break;
                //     case AssignmentDeclarationKind.Prototype:
                //         bindPrototypeAssignment(node as BindableStaticPropertyAssignmentExpression);
                //         break;
                //     case AssignmentDeclarationKind.ThisProperty:
                //         bindThisPropertyAssignment(node as BindablePropertyAssignmentExpression);
                //         break;
                //     case AssignmentDeclarationKind.Property:
                //         const expression = ((node as BinaryExpression).left as AccessExpression).expression;
                //         if (isInJSFile(node) && isIdentifier(expression)) {
                //             const symbol = lookupSymbolForName(blockScopeContainer, expression.escapedText);
                //             if (isThisInitializedDeclaration(symbol?.valueDeclaration)) {
                //                 bindThisPropertyAssignment(node as BindablePropertyAssignmentExpression);
                //                 break;
                //             }
                //         }
                //         bindSpecialPropertyAssignment(node as BindablePropertyAssignmentExpression);
                //         break;
                //     case AssignmentDeclarationKind.None:
                //         // Nothing to do
                //         break;
                //     default:
                //         Debug.fail("Unknown binary expression special property assignment kind");
                // }
                // return checkStrictModeBinaryExpression(node as BinaryExpression);
            }
            AstKind::CatchClause(catch) => {
                // return checkStrictModeCatchClause(node as CatchClause);
            }
            AstKind::UnaryExpression(expr) => {
                // if delete expression
                // return checkStrictModeDeleteExpression(node as DeleteExpression);
            }
            AstKind::UpdateExpression(expr) => {
                if !expr.prefix {
                // return checkStrictModePostfixUnaryExpression(node);
                } else{
                // return checkStrictModePrefixUnaryExpression(node);
                }
            }
            AstKind::WithStatement(stmt) => {
                // return checkStrictModeWithStatement(node as WithStatement);
            }
            AstKind::LabeledStatement(stmt) => {
                // return checkStrictModeLabeledStatement(node as LabeledStatement);
            }
            AstKind::TSThisType(this) => {
                // seenThisKeyword = true;
            }
            AstKind::TSTypePredicate(pred) => {
                // noop // Binding the children will handle everything
            }
            AstKind::TSTypeParameter(param) => {
                // return bindTypeParameter(node as TypeParameterDeclaration);
            }
            AstKind::FormalParameter(param) => {
                // return bindParameter(node as ParameterDeclaration);
            }
            AstKind::VariableDeclaration(decl) => {
                // return bindVariableDeclarationOrBindingElement(node as VariableDeclaration);
            }
            AstKind::BindingPattern(pattern) => {
                node.set_flowNode(self.currentFlow.clone());
            }
            AstKind::PropertyDefinition(prop) => {
                // return bindPropertyWorker(node);
            }
            AstKind::TSPropertySignature(prop) => {
                // return bindPropertyWorker(node);
            }
            AstKind::ObjectProperty(prop) => {
                // return bindPropertyOrMethodOrAccessor(node as Declaration, SymbolFlags.Property, SymbolFlags.PropertyExcludes);
            }
            AstKind::TSEnumMember(member) => {
                // return bindPropertyOrMethodOrAccessor(node as Declaration, SymbolFlags.EnumMember, SymbolFlags.EnumMemberExcludes);
            }
            AstKind::TSCallSignatureDeclaration(_) |
            AstKind::TSConstructSignatureDeclaration(_) |
            AstKind::TSIndexSignature(_) => {
                // return declareSymbolAndAddToSymbolTable(node as Declaration, SymbolFlags.Signature, SymbolFlags.None);
            }
            AstKind::MethodDefinition(_) |
            AstKind::TSMethodSignature(_) => {
                // If this is an ObjectLiteralExpression method, then it sits in the same space
                // as other properties in the object literal. So we use SymbolFlags.PropertyExcludes
                // so that it will conflict with any other object literal members with the same
                // name.
                // return bindPropertyOrMethodOrAccessor(node as Declaration, SymbolFlags.Method | ((node as MethodDeclaration).questionToken ? SymbolFlags.Optional : SymbolFlags.None), isObjectLiteralMethod(node) ? SymbolFlags.PropertyExcludes : SymbolFlags.MethodExcludes);
            }
            AstKind::Function(func) => {
                // return bindFunctionDeclaration(node as FunctionDeclaration);
            }
            // case SyntaxKind.Constructor:
            //     return declareSymbolAndAddToSymbolTable(node as Declaration, SymbolFlags.Constructor, /*symbolExcludes:*/ SymbolFlags.None);
            AstKind::AccessorProperty(prop) => {
                // if get accessor
                // return bindPropertyOrMethodOrAccessor(node as Declaration, SymbolFlags.GetAccessor, SymbolFlags.GetAccessorExcludes);
                // if set accessor
                // return bindPropertyOrMethodOrAccessor(node as Declaration, SymbolFlags.SetAccessor, SymbolFlags.SetAccessorExcludes);
            }
            AstKind::TSFunctionType(_) |
            // case SyntaxKind.JSDocFunctionType:
            // case SyntaxKind.JSDocSignature:
            AstKind::TSConstructorType(_) => {
                // return bindFunctionOrConstructorType(node as SignatureDeclaration | JSDocSignature);
            }
            AstKind::TSTypeLiteral(_) |
            // case SyntaxKind.JSDocTypeLiteral:
            AstKind::TSMappedType(_) => {
                // return bindAnonymousTypeWorker(node as TypeLiteralNode | MappedTypeNode | JSDocTypeLiteral);
            }
            // case SyntaxKind.JSDocClassTag:
            //     return bindJSDocClassTag(node as JSDocClassTag);
            AstKind::ObjectExpression(obj) => {
                // return bindObjectLiteralExpression(node as ObjectLiteralExpression);
            }
            AstKind::ArrowFunctionExpression(arrow) => {
                // return bindFunctionExpression(node as FunctionExpression | ArrowFunction);
            }
            AstKind::CallExpression(call) => {
                // const assignmentKind = getAssignmentDeclarationKind(node as CallExpression);
                // switch (assignmentKind) {
                //     case AssignmentDeclarationKind.ObjectDefinePropertyValue:
                //         return bindObjectDefinePropertyAssignment(node as BindableObjectDefinePropertyCall);
                //     case AssignmentDeclarationKind.ObjectDefinePropertyExports:
                //         return bindObjectDefinePropertyExport(node as BindableObjectDefinePropertyCall);
                //     case AssignmentDeclarationKind.ObjectDefinePrototypeProperty:
                //         return bindObjectDefinePrototypeProperty(node as BindableObjectDefinePropertyCall);
                //     case AssignmentDeclarationKind.None:
                //         break; // Nothing to do
                //     default:
                //         return Debug.fail("Unknown call expression assignment declaration kind");
                // }
                // if (isInJSFile(node)) {
                //     bindCallExpression(node as CallExpression);
                // }
            }
            AstKind::Class(class) => {
                // All classes are automatically in strict mode in ES6.
                self.inStrictMode = true;
                // return bindClassLikeDeclaration(node as ClassLikeDeclaration);
            }
            AstKind::TSInterfaceDeclaration(iface) => {
                // return bindBlockScopedDeclaration(node as Declaration, SymbolFlags.Interface, SymbolFlags.InterfaceExcludes);
            }
            AstKind::TSTypeAliasDeclaration(alias) => {
                // return bindBlockScopedDeclaration(node as Declaration, SymbolFlags.TypeAlias, SymbolFlags.TypeAliasExcludes);
            }
            AstKind::TSEnumDeclaration(enum_) => {
                // return bindEnumDeclaration(node as EnumDeclaration);
            }
            AstKind::TSModuleDeclaration(module) => {
                // return bindModuleDeclaration(node as ModuleDeclaration);
            }
            // case SyntaxKind.JsxAttributes:
            //     return bindJsxAttributes(node as JsxAttributes);
            AstKind::JSXAttribute(attr) => {
                // return bindJsxAttribute(node as JsxAttribute, SymbolFlags.Property, SymbolFlags.PropertyExcludes);
            }
            // Imports and exports
            AstKind::TSImportEqualsDeclaration(_) |
            AstKind::ImportNamespaceSpecifier(_) |
            AstKind::ImportSpecifier(_) |
            AstKind::ExportSpecifier(_) => {
                // return declareSymbolAndAddToSymbolTable(node as Declaration, SymbolFlags.Alias, SymbolFlags.AliasExcludes);
            }
            AstKind::TSNamespaceExportDeclaration(export_ns) => {
                // return bindNamespaceExportDeclaration(node as NamespaceExportDeclaration);
            }
            AstKind::ImportSpecifier(_) | AstKind::ImportDefaultSpecifier(_) | AstKind::ImportNamespaceSpecifier(_) => {
                // return bindImportClause(node as ImportClause);
            }
            AstKind::ExportNamedDeclaration(export_named) => {
                // return bindExportDeclaration(node as ExportDeclaration);
            }
            AstKind::TSExportAssignment(export_assign) => {
                // return bindExportAssignment(node as ExportAssignment);
            }
            AstKind::SourceFile(source_file) => {
                // updateStrictModeStatementList((node as SourceFile).statements);
                // return bindSourceFileIfExternalModule();
            }
            AstKind::BlockStatement(block) => {
                // if (!isFunctionLikeOrClassStaticBlockDeclaration(node.parent)) {
                //     return;
                // }
                // falls through
            }
            AstKind::TSModuleBlock(module_block) => {
                // return updateStrictModeStatementList((node as Block | ModuleBlock).statements);
            }
            // ! skipping jsdoc
            // case SyntaxKind.JSDocParameterTag:
            //     if (node.parent.kind === SyntaxKind.JSDocSignature) {
            //         return bindParameter(node as JSDocParameterTag);
            //     }
            //     if (node.parent.kind !== SyntaxKind.JSDocTypeLiteral) {
            //         break;
            //     }
            //     // falls through
            // case SyntaxKind.JSDocPropertyTag:
            //     const propTag = node as JSDocPropertyLikeTag;
            //     const flags = propTag.isBracketed || propTag.typeExpression && propTag.typeExpression.type.kind === SyntaxKind.JSDocOptionalType ?
            //         SymbolFlags.Property | SymbolFlags.Optional :
            //         SymbolFlags.Property;
            //     return declareSymbolAndAddToSymbolTable(propTag, flags, SymbolFlags.PropertyExcludes);
            // case SyntaxKind.JSDocTypedefTag:
            // case SyntaxKind.JSDocCallbackTag:
            // case SyntaxKind.JSDocEnumTag:
            //     return (delayedTypeAliases || (delayedTypeAliases = [])).push(node as JSDocTypedefTag | JSDocCallbackTag | JSDocEnumTag);
            // case SyntaxKind.JSDocOverloadTag:
            //     return bind((node as JSDocOverloadTag).typeExpression);
            // case SyntaxKind.JSDocImportTag:
            //     return (jsDocImports || (jsDocImports = [])).push(node as JSDocImportTag);
            _ => {}
        }
    }
    // endregion: 2082
}

// region: 3888
/** @internal */
pub fn getContainerFlags(node: &AstKind) -> ContainerFlags {
    let flags = match node {
        AstKind::Class(_) |
        AstKind::TSEnumDeclaration(_) |
        AstKind::ObjectExpression(_) |
        AstKind::TSTypeLiteral(_) |
            // ! skipping jsdoc
        // case SyntaxKind.JSDocTypeLiteral:
        AstKind::JSXAttribute(_) => Some(ContainerFlags::IsContainer),

        AstKind::TSInterfaceDeclaration(_) => Some(ContainerFlags::IsContainer | ContainerFlags::IsInterface),

        AstKind::TSModuleDeclaration(_) |
        AstKind::TSTypeAliasDeclaration(_) |
        AstKind::TSMappedType(_) |
        AstKind::TSIndexSignature(_) => Some(ContainerFlags::IsContainer | ContainerFlags::HasLocals),

        AstKind::SourceFile(_) => Some(ContainerFlags::IsContainer | ContainerFlags::IsControlFlowContainer | ContainerFlags::HasLocals),
        _ => None,
    };
    if let Some(flags) = flags {
        return flags;
    }

    if isObjectLiteralOrClassExpressionMethodOrAccessor(node) {
        return ContainerFlags::IsContainer | ContainerFlags::IsControlFlowContainer | ContainerFlags::HasLocals | ContainerFlags::IsFunctionLike | ContainerFlags::IsObjectLiteralOrClassExpressionMethodOrAccessor;
    }
    let flags = match node {
        AstKind::MethodDefinition(_) |
        AstKind::Function(_) |
        AstKind::TSMethodSignature(_) |
        AstKind::TSCallSignatureDeclaration(_) |
            // ! skipping jsdoc
        // case SyntaxKind.JSDocSignature:
        // case SyntaxKind.JSDocFunctionType:
        AstKind::TSFunctionType(_) |
        AstKind::TSConstructSignatureDeclaration(_) |
        AstKind::TSConstructorType(_) |
        AstKind::StaticBlock(_) => Some(ContainerFlags::IsContainer | ContainerFlags::IsControlFlowContainer | ContainerFlags::HasLocals | ContainerFlags::IsFunctionLike),
        _ => None,
    };
    if let Some(flags) = flags {
        return flags;
    }

    match node {
        AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) => ContainerFlags::IsContainer | ContainerFlags::IsControlFlowContainer | ContainerFlags::HasLocals | ContainerFlags::IsFunctionLike | ContainerFlags::IsFunctionExpression,

        AstKind::TSModuleBlock(_) => ContainerFlags::IsControlFlowContainer,
        AstKind::PropertyDefinition(n) => {
            if n.value.is_some() {
                ContainerFlags::IsControlFlowContainer
            } else {
                ContainerFlags::None
            }
        }

        AstKind::CatchClause(_) | AstKind::ForStatement(_) | AstKind::ForInStatement(_) | AstKind::ForOfStatement(_) | AstKind::SwitchStatement(_) => ContainerFlags::IsBlockScopedContainer | ContainerFlags::HasLocals,

        AstKind::BlockStatement(_) => {
            // do not treat blocks directly inside a function as a block-scoped-container.
            // Locals that reside in this block should go to the function locals. Otherwise 'x'
            // would not appear to be a redeclaration of a block scoped local in the following
            // example:
            //
            //      function foo() {
            //          var x;
            //          let x;
            //      }
            //
            // If we placed 'var x' into the function locals and 'let x' into the locals of
            // the block, then there would be no collision.
            //
            // By not creating a new block-scoped-container here, we ensure that both 'var x'
            // and 'let x' go into the Function-container's locals, and we do get a collision
            // conflict.
            if isFunctionLike(node.parent().as_ref()) || isClassStaticBlockDeclaration(node.parent().as_ref()) {
                ContainerFlags::None
            } else {
                ContainerFlags::IsBlockScopedContainer | ContainerFlags::HasLocals
            }
        }

        _ => ContainerFlags::None,
    }
}
// endregion: 3968
