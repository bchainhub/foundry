pragma solidity >=1.1.0;
pragma solidity >=1.1.0;

contract SimpleStorage {

    event ValueChanged(address indexed author, string oldValue, string newValue);

    string _value;

    constructor(string memory value) public {
        _value = value;
    }

    function getValue() view public returns (string memory) {
        return _value;
    }

    function setValue(string memory value) public {
        emit ValueChanged(msg.sender, _value, value);
        _value = value;
    }
}

contract SimpleStorage {

    event ValueChanged(address indexed author, string oldValue, string newValue);

    string _value;

    constructor(string memory value) public {
        _value = value;
    }

    function getValue() view public returns (string memory) {
        return _value;
    }

    function setValue(string memory value) public {
        emit ValueChanged(msg.sender, _value, value);
        _value = value;
    }
}
