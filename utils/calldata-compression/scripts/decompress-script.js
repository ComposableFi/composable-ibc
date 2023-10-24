class CompressDataDescription {
    constructor(startByte, amountBytes, method) {
        this.startByte = startByte;
        this.amountBytes = amountBytes;
        this.method = method;
    }
}
class CompressDataPower {
    constructor(decompressedBytesAmount, compressedBytesAmount) {
        this.decompressedSize = decompressedBytesAmount;
        this.compressedSize = compressedBytesAmount;
    }

    range() {
        return this.decompressedSize - this.compressedSize;
    }

    add(c) {
        this.decompressedSize += c.decompressedSize;
        this.compressedSize += c.compressedSize;
    }
}
class CompressData {
    constructor(power, description) {
        this.power = power;
        this.description = description;
    }
}
class Calldata {
    constructor(data) {
        data = data.replace(/^0x+/, '').toLowerCase();
        const dataTrim0 = data.replace(/^0+/, '').toLowerCase();
        if (BigInt('0x' + data).toString(16) !== (dataTrim0 === '' ? '0' : dataTrim0)) {
            throw Error('The data is not hexadecimal');
        }
        if (data.length % 2 !== 0) {
            throw Error('Wrong data length');
        }
        this.data = data;
        // this.contract = decompressorExtension;
        this.bytesInfo = [];
    }

    analyse() {
        this.bytesInfo = [];
        for (let i = 0; i < this.data.length; i += 2) {
            this.bytesInfo.push({
                index: i / 2,
                zeroCompress: this.checkZerosCase(i / 2),
                copyCompress: this.checkCopyCaseWithZeros(i / 2),
                storageCompress: this.checkStorageCase(i / 2),
            });
        }
        return this.bytesInfo;
    }

    #compressPart(fromByte, toByte) {
        function createDesc(arrayDesc, amountBytes, method) {
            let startByte = fromByte;
            if (arrayDesc.length !== 0) {
                const prevDescIndex = arrayDesc.length - 1;
                startByte = arrayDesc[prevDescIndex].startByte + arrayDesc[prevDescIndex].amountBytes;
            }
            return new CompressDataDescription(
                startByte,
                amountBytes,
                method,
            );
        }

        function addJustCopyCompress(resultCompress, amount) {
            if (amount !== 0) {
                resultCompress.power.add(new CompressDataPower(amount, 1 + amount));
                resultCompress.description.push(
                    createDesc(resultCompress.description, amount, '01'),
                );
            }
            return resultCompress;
        }

        let partCompress = new CompressData(
            new CompressDataPower(0, 0),
            [],
        );

        let justCopyAmount = 0;
        for (let i = fromByte; i <= toByte;) {
            if (this.bytesInfo[i].zeroCompress.decompressedSize >= toByte - i + 1) {
                partCompress = addJustCopyCompress(partCompress, justCopyAmount);
                partCompress.power.add(new CompressDataPower(toByte - fromByte + 1, 1));
                partCompress.description.push(
                    new CompressDataDescription(
                        i,
                        toByte - i + 1,
                        '00',
                    ),
                );
                return partCompress;
            }

            let zeroBytesAmount = 0;
            let isPaddingWithCopy = false;
            let needJustCopyAmount = true;

            if (this.bytesInfo[i].zeroCompress.decompressedSize !== 0) {
                if (this.bytesInfo[i].copyCompress.decompressedSize >= toByte - i + 1 ||
                    this.bytesInfo[i].zeroCompress.range() > this.bytesInfo[i].copyCompress.range()) {
                    zeroBytesAmount = this.bytesInfo[i].zeroCompress.decompressedSize;
                } else {
                    isPaddingWithCopy = true;
                }
            }

            let isStorageCompressUsed = false;
            const isZeroCompress = zeroBytesAmount > 0;
            for (let j = 0; j < this.bytesInfo[i].storageCompress.length; j++) {
                if (this.bytesInfo[i].storageCompress[j].decompressedSize <= toByte - i + 1) {
                    const isStorageRangeMoreThanCopyCompress = this.bytesInfo[i].storageCompress[j].range() > this.bytesInfo[i].copyCompress.range();

                    if (!isZeroCompress && !isStorageRangeMoreThanCopyCompress && !isPaddingWithCopy) {
                        continue;
                    }

                    partCompress = addJustCopyCompress(partCompress, justCopyAmount);

                    if (isZeroCompress) {
                        if (this.bytesInfo[i].storageCompress[j].range() > this.bytesInfo[i].zeroCompress.range()) {
                            partCompress.power.add(this.bytesInfo[i].storageCompress[j]);
                            partCompress.description.push(
                                createDesc(
                                    partCompress.description,
                                    this.bytesInfo[i].storageCompress[j].decompressedSize,
                                    this.bytesInfo[i].storageCompress[j].compressedSize === 2 ? '10' : '11',
                                ),
                            );
                            i += this.bytesInfo[i].storageCompress[j].decompressedSize;
                        } else {
                            partCompress.power.add(this.bytesInfo[i].zeroCompress);
                            partCompress.description.push(
                                createDesc(
                                    partCompress.description,
                                    zeroBytesAmount,
                                    '00',
                                ),
                            );
                            i += zeroBytesAmount;
                        }
                    } else if (isStorageRangeMoreThanCopyCompress) {
                        partCompress.power.add(this.bytesInfo[i].storageCompress[j]);
                        partCompress.description.push(
                            createDesc(
                                partCompress.description,
                                this.bytesInfo[i].storageCompress[j].decompressedSize,
                                this.bytesInfo[i].storageCompress[j].compressedSize === 2 ? '10' : '11',
                            ),
                        );
                        i += this.bytesInfo[i].storageCompress[j].decompressedSize;
                    } else if (isPaddingWithCopy) {
                        partCompress.power.add(this.bytesInfo[i].copyCompress);
                        partCompress.description.push(
                            createDesc(
                                partCompress.description,
                                this.bytesInfo[i].copyCompress.decompressedSize,
                                '01',
                            ),
                        );
                        i += this.bytesInfo[i].copyCompress.decompressedSize;
                    }

                    justCopyAmount = 0;
                    needJustCopyAmount = false;
                    isStorageCompressUsed = true;
                    break;
                }
            }

            if (!isStorageCompressUsed) {
                if (isZeroCompress || isPaddingWithCopy) {
                    partCompress = addJustCopyCompress(partCompress, justCopyAmount);
                }

                if (isZeroCompress) {
                    partCompress.power.add(this.bytesInfo[i].zeroCompress);
                    partCompress.description.push(
                        createDesc(
                            partCompress.description,
                            zeroBytesAmount,
                            '00',
                        ),
                    );
                    i += zeroBytesAmount;
                } else if (isPaddingWithCopy) {
                    partCompress.power.add(this.bytesInfo[i].copyCompress);
                    partCompress.description.push(
                        createDesc(
                            partCompress.description,
                            this.bytesInfo[i].copyCompress.decompressedSize,
                            '01',
                        ),
                    );
                    i += this.bytesInfo[i].copyCompress.decompressedSize;
                }

                if (isZeroCompress || isPaddingWithCopy) {
                    justCopyAmount = 0;
                    needJustCopyAmount = false;
                }
            }

            if (needJustCopyAmount) {
                const newJustCopyAmount = Math.min(this.bytesInfo[i].copyCompress.decompressedSize, toByte - i + 1);
                justCopyAmount += newJustCopyAmount;
                if (justCopyAmount > 32) {
                    partCompress = addJustCopyCompress(partCompress, 32);
                    justCopyAmount -= 32;
                }
                i += newJustCopyAmount;
            }
        }

        partCompress = addJustCopyCompress(partCompress, justCopyAmount);

        return partCompress;
    }

    zip(instractions) {
        function scaleFraction(fraction) {
            if (fraction.length % 2 !== 0) {
                fraction = '0' + fraction;
            }
            return fraction;
        }

        let result = '0x';
        const BB = [32, 20, 4, 31];
        let index = 0;
        for (let i = 0; i < instractions.length; i++) {
            switch (instractions[i].method) {
                case '00':
                    // 00XXXXXX
                    result += scaleFraction(BigInt(instractions[i].amountBytes - 1).toString(16));
                    break;
                case '01': {
                    // 01PXXXXX
                    const copyBytes = this.getBytes(instractions[i].startByte, instractions[i].amountBytes);
                    let nonZeroByteIndex = 0;
                    for (let j = 0; j < instractions[i].amountBytes; j++) {
                        if (copyBytes.substring(j * 2, j * 2 + 2) !== '00') {
                            nonZeroByteIndex = j;
                            break;
                        }
                    }
                    result += scaleFraction(BigInt((instractions[i].amountBytes - nonZeroByteIndex - 1) + 64 + (nonZeroByteIndex === 0 ? 0 : 32)).toString(16));
                    result += this.getBytes(instractions[i].startByte + nonZeroByteIndex, instractions[i].amountBytes - nonZeroByteIndex);
                    break;
                }
                case '10':
                    // 10BBXXXX XXXXXXXX
                    index = this.lookup[this.getBytes(instractions[i].startByte, instractions[i].amountBytes)];
                    result += scaleFraction(BigInt(index + 2 ** 15 + (BB.indexOf(instractions[i].amountBytes) * 2 ** 12)).toString(16));
                    break;
                case '11':
                    // 11BBXXXX XXXXXXXX XXXXXXXX
                    index = this.lookup[this.getBytes(instractions[i].startByte, instractions[i].amountBytes)];
                    result += scaleFraction(BigInt(index + 3 * 2 ** 22 + (BB.indexOf(instractions[i].amountBytes) * 2 ** 20)).toString(16));
                    break;
                default:
                    throw Error('Unsupported method: ' + instractions[i].method);
            }
        }
        return result;
    }

    compress() {
        this.analyse();

        const bestCompressForFirstNBytes = [];
        if (this.bytesInfo[0].zeroCompress.decompressedSize !== 0) {
            bestCompressForFirstNBytes[0] = new CompressData(
                new CompressDataPower(1, 1),
                [new CompressDataDescription(0, 1, '00')],
            );
        } else {
            bestCompressForFirstNBytes[0] = new CompressData(
                new CompressDataPower(1, 2),
                [new CompressDataDescription(0, 1, '01')],
            );
        }

        for (let i = 1; i < this.bytesInfo.length; i++) {
            bestCompressForFirstNBytes[i] = new CompressData(
                new CompressDataPower(
                    bestCompressForFirstNBytes[i - 1].power.decompressedSize + 1,
                    bestCompressForFirstNBytes[i - 1].power.compressedSize + 2,
                ),
                [
                    ...bestCompressForFirstNBytes[i - 1].description,
                    new CompressDataDescription(i, 1, '01'),
                ],
            );

            for (let j = i; j >= Math.max(0, i - 63); j--) {
                const partCompress = this.#compressPart(j, i);

                const prefixCompress = new CompressData(
                    new CompressDataPower(0, 0),
                    [],
                );
                if (partCompress.description[0].startByte !== 0) {
                    prefixCompress.power = bestCompressForFirstNBytes[partCompress.description[0].startByte - 1].power;
                    prefixCompress.description = bestCompressForFirstNBytes[partCompress.description[0].startByte - 1].description;
                }

                if (prefixCompress.power.range() + partCompress.power.range() > bestCompressForFirstNBytes[i].power.range()) {
                    bestCompressForFirstNBytes[i] = new CompressData(
                        new CompressDataPower(
                            prefixCompress.power.decompressedSize + partCompress.power.decompressedSize,
                            prefixCompress.power.compressedSize + partCompress.power.compressedSize,
                        ),
                        [...prefixCompress.description, ...partCompress.description],
                    );
                }
            }
        }

        return {
            uncompressedData: '0x' + this.data,
            compressedData: this.zip(bestCompressForFirstNBytes[this.bytesInfo.length - 1].description),
            ...bestCompressForFirstNBytes[this.bytesInfo.length - 1],
        };
    }

    getByte(n) {
        return this.getBytes(n, 1);
    }

    getBytes(start, n = 1) {
        return this.data.slice(2 * start, 2 * (start + n));
    }

    async initDict() {
        this.lookup = {
            '000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266': 0,
            ffb92266: 0,
            f39fd6e51aad88f6f4ce6ab8827279cfffb92266: 0,
            '0000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb92266': 0,
            '0000000000000000000000005fbdb2315678afecb367f032d93f642f64180aa3': 1,
            '64180aa3': 1,
            '5fbdb2315678afecb367f032d93f642f64180aa3': 1,
            '00000000000000000000005fbdb2315678afecb367f032d93f642f64180aa3': 1
        };
    }

    checkZerosCase(n) {
        let currentByteIndex = n;
        const byte = this.getByte(currentByteIndex);
        if (byte !== '00') {
            return new CompressDataPower(0, 0);
        }
        currentByteIndex++;
        while (this.getByte(currentByteIndex) === '00' && currentByteIndex < this.data.length / 2 && currentByteIndex - n <= 63) {
            currentByteIndex++;
        }
        return new CompressDataPower(currentByteIndex - n, 1);
    }

    checkCopyCaseWithZeros(n) {
        let currentByteIndex = n;
        const byte = this.getByte(currentByteIndex);
        if (byte !== '00') {
            return new CompressDataPower(1, 2);
        }
        currentByteIndex++;
        while (this.getByte(currentByteIndex) === '00' && currentByteIndex < this.data.length) {
            if (currentByteIndex - n === 32) {
                return new CompressDataPower(31, 32);
            }
            currentByteIndex++;
        }
        const decompressedBytesAmount = Math.min(this.data.length / 2 - n, 32);
        return new CompressDataPower(decompressedBytesAmount, decompressedBytesAmount === 32 ? 1 + 32 - (currentByteIndex - n + 1) : 1 + decompressedBytesAmount);
    }

    checkStorageCase(n) {
        const best = [];
        for (const len of [32, 31, 20, 4]) {
            const tail = this.getBytes(n, len);
            const index = this.lookup[tail];
            if (tail.length / 2 >= len && index) {
                best.push(new CompressDataPower(len, index > 4096 ? 3 : 2));
            }
        }
        return best;
    }
}

async function compress(calldata) {
    const calldataObj = new Calldata(calldata);
    await calldataObj.initDict();
    return calldataObj.compress();
}


async function initLoadCalldatas(inputPath) {
    let calldatas = {};
    try {
        calldatas = require(inputPath);
    } catch (e) {
        console.warn('\x1b[33m%s\x1b[0m', 'Warning: ', 'There is no tx-calldata.json');
    }
    return { calldatas };
};



const main = async () => {

    const inputPath = process.argv[2];
    const { calldatas } = await initLoadCalldatas(inputPath);
    const result = await compress(calldatas[0]);
    let compressedData = result.compressedData.replaceAll(/\s/g, '');

    var fs = require('fs');
    fs.writeFile("test.txt", compressedData, function (err) {
        if (err) {
            console.log(err);
        }
    });
}


main().catch((err) => {
    console.error(err);
    process.exit(1);
});
