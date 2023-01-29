import pytest
from poetry_rust_integration import ManagedThing

# def test_context_manager():
#     with ManagedThing(6, 7) as resource:
#         # this returns the result of an operation on the resource, not the resource itself
#         x = resource()
#         assert isinstance(x, int)
#         assert x == 42

#     # not accessible outside cm
#     resource = ManagedThing(0, 1)
#     with pytest.raises(RuntimeError):
#         resource()


if __name__ == "__main__":
    # test_context_manager()

    with ManagedThing(6, 7) as cm:
        print(cm())
    try:
        print(cm())
    except ReferenceError as e:
        print(f"as expected: {e}")
    else:
        raise ReferenceError("context-managed object available outside context manager")

    cm2 = ManagedThing(6, 7)
    try:
        print(cm2())
    except ReferenceError as e:
        print(f"as expected: {e}")
    else:
        raise ReferenceError("context-managed object available outside context manager")
