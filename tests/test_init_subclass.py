from poetry_rust_integration import Registry


def test_registry() -> None:
    class PyBase:
        """Some unrelated base class"""

    class Test0(Registry, PyBase):
        pass

    class Test1(Registry, PyBase, id=5):
        pass

    class Test2(Registry, PyBase, id=8):
        pass

    class Test3(PyBase, Registry, id=13, other="string"):
        pass

    assert Test0 in Registry.list
    assert Registry.list[Test0] == {}
    assert Registry.list[Test1]["id"] == 5
    assert Registry.list[Test2] == {"id": 8}
    assert Registry.list[Test3] == {"id": 13, "other": "string"}


if __name__ == "__main__":
    test_registry()
