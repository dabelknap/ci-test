import pytest
import mylib

def test_english():
    greeter = mylib.Greet("world")
    assert greeter.greet() == "Hello world!"

def test_french():
    greeter = mylib.Greet("monde")
    assert greeter.saluer() == "Bonjour monde!"

def test_japanese():
    greeter = mylib.Greet("sekai")
    assert greeter.japanese() == "Konichiwa sekai!"
