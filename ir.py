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
            value JSON NOT NULL
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

    def _get_expr(self, expr: ParameterExpression) -> int:
        # constant folding
        match expr:
            case Add(lhs, rhs):
                if isinstance(lhs, Constant) and isinstance(rhs, Constant):
                    expr = Constant(lhs.value + rhs.value)
            case Sub(lhs, rhs):
                if isinstance(lhs, Constant) and isinstance(rhs, Constant):
                    expr = Constant(lhs.value - rhs.value)

        # check if the expression is already in the database
        cursor = self._exprs.cursor()
        expr_json = json.dumps(dataclasses.asdict(expr))
        cursor.execute("SELECT id FROM exprs WHERE value = ? LIMIT 1", (expr_json,))
        result = cursor.fetchone()
        if result:
            return result[0]
        cursor.execute("INSERT INTO exprs VALUES (?)", (expr_json,))
        self._exprs.commit()
        return cursor.lastrowid

    def from_ast(self, ast_module: dict):
        self._name = ast_module["name"]

        # initialize parameters
        for param in ast_module["params"]:
            self._params[param["name"]] = Constant(param["value"])

        # initialize ports
        for statement in ast_module["body"]:
            if "Wire" in statement: # wire declaration
                wire = statement["Wire"]
                if wire["io"] is None:  # internal wire
                    continue
                tensor = BitTensor()
                tensor.from_ast(self, wire)
                if wire["io"] in ["Input", "InOut"]:    # we treat InOut as Input for now
                    self._inputs[wire["name"]] = tensor
                elif wire["io"] == "Output":
                    self._outputs[wire["name"]] = tensor
                else:
                    raise DynStructIRError(f"Unknown io type: {wire['io']}")


class ParameterExpression:
    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        # compare by field addresses if they are ParameterExpressions
        for field_name, field_value in vars(self).items():
            other_field_value = getattr(other, field_name)
            if isinstance(field_value, ParameterExpression) and isinstance(other_field_value, ParameterExpression):
                if id(field_value) != id(other_field_value):
                    return False
            elif field_value != other_field_value:
                return False
        return True

    def __hash__(self):
        field_hashes = tuple(sorted((field_name, id(getattr(self, field_name))) for field_name in vars(self)))
        return hash((self.__class__, field_hashes))

@dataclasses.dataclass
class Constant(ParameterExpression):
    value: int

@dataclasses.dataclass
class Parameter(ParameterExpression):
    name: str

@dataclasses.dataclass
class Add(ParameterExpression):
    lhs: ParameterExpression
    rhs: ParameterExpression

@dataclasses.dataclass
class Sub(ParameterExpression):
    lhs: ParameterExpression
    rhs: ParameterExpression


class BitTensor:
    _shape: list[ParameterExpression]
    _op: Op | None

    def __init__(self, shape: list[ParameterExpression] = None, op: Op = None):
        self._shape = shape or []
        self._op = op

    def __repr__(self):
        return f"BitTensor(shape={self._shape}, op={self._op})"

    def from_ast(self, module: Module, wire: dict):
        if wire["width"] is None:   # 1-bit wire
            self._shape = [module.]


class Op:
    pass


if __name__ == "__main__":
    MODULE_JSON_FILE = "simple_and.json"

    with open(MODULE_JSON_FILE, "r") as f:
        ast_module = json.load(f)

    module = Module()
    module.from_ast(ast_module)