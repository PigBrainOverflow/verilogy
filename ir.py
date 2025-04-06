# This is a tentative prototype of DynStructIR.

from __future__ import annotations

import json
import dataclasses
import sqlite3


class DynStructIRError(Exception):
    pass


class Module:
    _name: str | None
    _exprs: sqlite3.Connection
    _inputs: dict[str, BitTensor]
    _outputs: dict[str, BitTensor]

    CREATE_EXPR_TABLE = """
        CREATE TABLE IF NOT EXISTS exprs (
            id INTEGER PRIMARY KEY,
            content JSON NOT NULL
        );
    """

    def _init_expr_db(self) -> sqlite3.Connection:
        conn = sqlite3.connect(":memory:")
        conn.execute(self.CREATE_EXPR_TABLE)
        conn.commit()
        return conn

    def __init__(self):
        self._name = None
        self._exprs = self._init_expr_db()
        self._inputs = {}
        self._outputs = {}

    def __delete__(self):
        self._exprs.close()

    def _get_expr(self, expr: ParameterExpression) -> int:
        # constant folding
        match expr:
            case Add(lhs, rhs):
                if isinstance(lhs, Constant) and isinstance(rhs, Constant):
                    expr = Constant(lhs.value + rhs.value)
            case Sub(lhs, rhs):
                if isinstance(lhs, Constant) and isinstance(rhs, Constant):
                    expr = Constant(lhs.value - rhs.value)
            case Mul(lhs, rhs):
                if isinstance(lhs, Constant) and isinstance(rhs, Constant):
                    expr = Constant(lhs.value * rhs.value)

        # check if the expression is already in the database
        cursor = self._exprs.cursor()
        expr_json = json.dumps(expr.to_json())
        cursor.execute("SELECT id FROM exprs WHERE content = ? LIMIT 1", (expr_json,))
        result = cursor.fetchone()
        if result:
            return result[0]
        cursor.execute("INSERT INTO exprs (content) VALUES (?)", (expr_json,))
        self._exprs.commit()
        # print(f"Inserted new expression: {expr_json}")
        return cursor.lastrowid

    def from_ast(self, ast_module: dict):
        genvars = set()

        self._name = ast_module["name"]

        # initialize parameters
        for param in ast_module["params"]:
            self._get_expr(Parameter(param["name"]))

        for statement in ast_module["body"]:
            if "Wire" in statement: # wire declaration
                wire = statement["Wire"]
                if wire["io"] is None:  # internal wire
                    continue
                tensor = BitTensor()
                tensor.from_ast(self, wire)
                # print(tensor)
                if wire["io"] in ["Input", "InOut"]:    # we treat InOut as Input for now
                    self._inputs[wire["name"]] = tensor
                elif wire["io"] == "Output":
                    self._outputs[wire["name"]] = tensor
                else:
                    raise DynStructIRError(f"Unknown io type: {wire['io']}")

            elif "Genvar" in statement: # genvar declaration
                genvars.add(statement["Genvar"])

            elif "Generate" in statement:   # generate block
                pass


class ParameterExpression:
    @staticmethod
    def from_ast(module: Module, expr: dict) -> int:
        if "BinaryArithmeticOperation" in expr:
            lhs, op, rhs = expr["BinaryArithmeticOperation"]
            lhs = ParameterExpression.from_ast(module, lhs)
            rhs = ParameterExpression.from_ast(module, rhs)
            match op:
                case "Add":
                    return module._get_expr(Add(lhs, rhs))
                case "Sub":
                    return module._get_expr(Sub(lhs, rhs))
                case "Mul":
                    return module._get_expr(Mul(lhs, rhs))
                case _:
                    raise DynStructIRError(f"Unknown operation: {op}")
        if "ConstantInt" in expr:
            return module._get_expr(Constant(expr["ConstantInt"]))
        if "Identifier" in expr:
            return module._get_expr(Parameter(expr["Identifier"]))
        raise DynStructIRError(f"Unknown expression: {expr}")

    def to_json(self) -> dict:
        return {"type": self.__class__.__name__, **dataclasses.asdict(self)}


@dataclasses.dataclass
class Constant(ParameterExpression):
    value: int

@dataclasses.dataclass
class Parameter(ParameterExpression):
    name: str

@dataclasses.dataclass
class Add(ParameterExpression):
    lhs: int
    rhs: int

@dataclasses.dataclass
class Sub(ParameterExpression):
    lhs: int
    rhs: int

@dataclasses.dataclass
class Mul(ParameterExpression):
    lhs: int
    rhs: int


class BitTensor:
    _shape: list[int]
    _op: Op | None

    def __init__(self, shape: list[int] = None, op: Op = None):
        self._shape = shape or []
        self._op = op

    def __repr__(self):
        return f"BitTensor(shape={self._shape}, op={self._op})"

    def from_ast(self, module: Module, wire: dict):
        # it ignores init and io
        if wire["width"] is None:   # 1-bit wire
            self._shape = [module._get_expr(Constant(1))]
        else:
            start, end = wire["width"]["start"], wire["width"]["end"]
            if end is None or end["ConstantInt"] != 0:
                raise DynStructIRError("Invalid width for wire")
            self._shape = [ParameterExpression.from_ast(module, start)]


class Op:
    pass


if __name__ == "__main__":
    MODULE_JSON_FILE = "simple_and.json"

    with open(MODULE_JSON_FILE, "r") as f:
        ast_module = json.load(f)

    module = Module()
    module.from_ast(ast_module)
    cur = module._exprs.cursor()
    cur.execute("SELECT * FROM exprs")
    for row in cur:
        print(row)