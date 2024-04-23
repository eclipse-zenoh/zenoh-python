import ast
from collections import defaultdict
from pathlib import Path

PACKAGE = (Path(__file__) / "../../zenoh").resolve()
__INIT__ = PACKAGE / "__init__.py"


class RemoveOverload(ast.NodeTransformer):
    def __init__(self):
        self.current_cls = None
        self.overloaded: defaultdict[str | None, set[str]] = defaultdict(set)

    def visit_ClassDef(self, node: ast.ClassDef):
        self.current_cls = node.name
        res = self.generic_visit(node)
        self.current_cls = None
        return res

    def visit_FunctionDef(self, node: ast.FunctionDef):
        for decorator in node.decorator_list:
            if isinstance(decorator, ast.Name) and decorator.id == "overload":
                if node.name in self.overloaded[self.current_cls]:
                    if node.name in ("serializer", "deserializer"):
                        func = ast.parse(
                            f"def {node.name}(arg, /): {ast.unparse(node.body[0])}"
                        )
                        return [node, func]
                    return None
                self.overloaded[self.current_cls].add(node.name)
                if node.name in ("serializer", "deserializer"):
                    return node
                node.decorator_list.clear()
                if node.name in ("recv", "try_recv", "__iter__"):
                    node.decorator_list.clear()
                else:
                    assert isinstance(node.returns, ast.Subscript)
                    if isinstance(node.returns.slice, ast.Subscript):
                        tp = node.returns.slice.slice
                    else:
                        tp = node.returns.slice
                    assert isinstance(tp, ast.Name)
                    annotation = f"_RustHandler[{tp.id}] | tuple[Callable[[{tp.id}], Any], Any] | Callable[[{tp.id}], Any] | None"
                    for arg in (*node.args.args, *node.args.kwonlyargs):
                        if arg.arg == "handler":
                            arg.annotation = ast.parse(annotation)
                    node.returns = node.returns.value
        for arg in (*node.args.posonlyargs, *node.args.args, *node.args.kwonlyargs):
            if ann := arg.annotation:
                arg.annotation = ast.Constant(f"{ast.unparse(ann)}")
        if ret := node.returns:
            node.returns = ast.Constant(f"{ast.unparse(ret)}")
        return node


def main():
    __INIT__.unlink()
    for entry in PACKAGE.glob("*.pyi"):
        entry.rename(PACKAGE / f"{entry.stem}.py")
    with open(__INIT__) as f:
        code = ast.parse(f.read())
    with open(__INIT__, "w") as f:
        f.write(ast.unparse(RemoveOverload().visit(code)))
    # open(PACKAGE / "plop.py", "w").write(ast.unparse(RemoveOverload().visit(ast.parse(open(PACKAGE/ "__init__.pyi").read()))))


if __name__ == "__main__":
    main()
