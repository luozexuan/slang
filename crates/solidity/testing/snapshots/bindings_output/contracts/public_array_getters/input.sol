contract Test {
    function test(TokenState tokenState) public {
        tokenState.owners(1).balance;
    }
}
contract TokenState {
    address[] public owners;
}
