import pytest
from poetry_rust_integration import ManagedThing

def test_context_manager():
    with ManagedThing(6, 7) as resource:
        # this returns the result of an operation on the resource, not the resource itself
        x = resource()
        assert isinstance(x, int)
        assert x == 42

    # not accessible outside cm
    resource = ManagedThing(0, 1)
    with pytest.raises(ReferenceError):  # NB C++ implementation raises RuntimeError
        resource()


if __name__ == "__main__":
    test_context_manager()
