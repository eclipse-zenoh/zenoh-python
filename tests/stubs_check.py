#
# Copyright (c) 2024 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
import ast
import importlib
import inspect
from inspect import Parameter
from pathlib import Path
from typing import Any

PACKAGE = (Path(__file__) / "../../zenoh").resolve()


class CheckExported(ast.NodeVisitor):
    def __init__(self, module: Any):
        self.module = module
        self.current_cls = None

    def visit_ClassDef(self, node: ast.ClassDef):
        # register the current class for method name disambiguation
        self.current_cls = getattr(self.module, node.name)
        getattr(self.current_cls, "__repr__")
        self.generic_visit(node)
        self.current_cls = None

    def visit_FunctionDef(self, node: ast.FunctionDef):
        func = getattr(self.current_cls or self.module, node.name)
        if node.name.startswith("__") or node.name.endswith("serializer"):
            pass
        elif callable(func):
            sig_params = {
                p.name: (p.kind, p.default is not Parameter.empty)
                for p in inspect.signature(func).parameters.values()
            }
            node_params = {}
            for i, arg in enumerate(node.args.posonlyargs):
                node_params[arg.arg] = (
                    Parameter.POSITIONAL_ONLY,
                    len(node.args.defaults)
                    >= len(node.args.args) + len(node.args.posonlyargs) - i,
                )
            for i, arg in enumerate(node.args.args):
                node_params[arg.arg] = (
                    Parameter.POSITIONAL_OR_KEYWORD,
                    len(node.args.defaults) >= len(node.args.args) - i,
                )
            if arg := node.args.vararg:
                node_params[arg.arg] = (Parameter.VAR_POSITIONAL, False)
            for arg, default in zip(node.args.kwonlyargs, node.args.kw_defaults):
                node_params[arg.arg] = (Parameter.KEYWORD_ONLY, default is not None)
            if arg := node.args.kwarg:
                node_params[arg.arg] = (Parameter.VAR_KEYWORD, False)
            node_params.pop("cls", ...)
            if "self" in node_params:
                node_params["self"] = (Parameter.POSITIONAL_ONLY, False)
            if (param := node_params.get("handler")) and not param[1]:
                return
            assert (
                sig_params == node_params
            ), f"{self.current_cls=}\n{func=}\n{sig_params=}\n{node_params=}"
        else:
            getattr(func, "__get__")

    def visit_AnnAssign(self, node: ast.AnnAssign):
        if self.current_cls is not None:
            assert isinstance(node.target, ast.Name)
            getattr(self.current_cls, node.target.id)


def main():
    for entry in PACKAGE.glob("*.pyi"):
        with open(entry) as f:
            parts = list(entry.relative_to(PACKAGE.parent).parts)
            parts[-1] = parts[-1].rstrip(".pyi")
            module_name = ".".join(p for p in parts if p != "__init__")
            visitor = CheckExported(importlib.import_module(module_name))
            visitor.visit(ast.parse(f.read()))


if __name__ == "__main__":
    main()
