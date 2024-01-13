// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.12;

import {IIBCModule} from "../core/05-port/IIBCModule.sol";
import "../proto/Channel.sol";
import "../core/25-handler/IIBCHandler.sol";

abstract contract IBCAppBase is IIBCModule {
    /**
     * @dev Throws if called by any account other than the IBC contract.
     */
    modifier onlyIBC() {
        _checkIBC();
        _;
    }

    function _msgSender() internal view virtual returns (address) {
        return msg.sender;
    }

    function _msgData() internal view virtual returns (bytes calldata) {
        return msg.data;
    }

    /**
     * @dev Returns the address of the IBC contract.
     */
    function ibcAddress() public view virtual returns (address);

    /**
     * @dev Throws if the sender is not the IBC contract.
     */
    function _checkIBC() internal view virtual {
        require(
            ibcAddress() == _msgSender(),
            "IBCAppBase: caller is not the IBC contract"
        );
    }

    /**
     * @dev See IIBCModule-onChanOpenInit
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onChanOpenInit(
        Channel.Order,
        string[] calldata,
        string calldata,
        string calldata channelId,
        ChannelCounterparty.Data calldata,
        string calldata
    ) external virtual override onlyIBC {}

    function onChanOpenTry(
        string[] calldata connectionHops,
        string calldata portId,
        string calldata channelId,
        ChannelCounterparty.Data calldata counterparty,
        string calldata version,
        string calldata counterpartyVersion
    ) external virtual override onlyIBC {}

    /**
     * @dev See IIBCModule-onChanOpenAck
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onChanOpenAck(
        string calldata portId,
        string calldata channelId,
        string calldata counterpartyVersion
    ) external virtual override onlyIBC {}

    /**
     * @dev See IIBCModule-onChanOpenConfirm
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onChanOpenConfirm(
        string calldata portId,
        string calldata channelId
    ) external virtual override onlyIBC {}

    /**
     * @dev See IIBCModule-onChanCloseInit
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onChanCloseInit(
        string calldata portId,
        string calldata channelId
    ) external virtual override onlyIBC {}

    /**
     * @dev See IIBCModule-onChanCloseConfirm
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onChanCloseConfirm(
        string calldata portId,
        string calldata channelId
    ) external virtual override onlyIBC {}

    /**
     * @dev See IIBCModule-onRecvPacket
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onRecvPacket(
        Packet.Data calldata packet,
        address
    )
        external
        virtual
        override
        onlyIBC
        returns (bytes memory acknowledgement)
    {
        return packet.data;
    }

    /**
     * @dev See IIBCModule-onAcknowledgementPacket
     *
     * NOTE: You should apply an `onlyIBC` modifier to the function if a derived contract overrides it.
     */
    function onAcknowledgementPacket(
        Packet.Data calldata,
        bytes calldata,
        address
    ) external virtual override onlyIBC {}
}

contract MockModule is IBCAppBase {
    address private _ibcAddress;

    constructor(address ibcAddress_) {
        _ibcAddress = ibcAddress_;
    }

    function ibcAddress() public view override returns (address) {
        return _ibcAddress;
    }

    function sendMockPacket(
        bytes memory data,
        string memory sourcePort,
        string memory sourceChannel,
        uint64 timeoutHeight
    ) external {
        IIBCHandler(ibcAddress()).sendPacket(
            sourcePort,
            sourceChannel,
            Height.Data({revision_number: 0, revision_height: timeoutHeight}),
            0,
            data
        );
    }
}
