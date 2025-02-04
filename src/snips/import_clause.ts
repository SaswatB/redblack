// 1) Default import only
// @ts-ignore
import React from "react";

// 2) Named imports only
// @ts-ignore
import { useState, useEffect } from "react";

// 3) Default + Named imports
// @ts-ignore
import ReactDOM, { render } from "react-dom";

// 4) Namespace import (all-import)
// @ts-ignore
import * as MyLib from "./MyLib";

// 5) Named import with aliases
// @ts-ignore
import { a as alpha, b, c as charlie } from "./AnotherLib";

// 6) Import type only (TypeScript 3.8+)
// @ts-ignore
import type { SomeType } from "./types";

// 7) Side-effect import (no ImportClause at all)
import "./setup";
