// automatically generated by the FlatBuffers compiler, do not modify
/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */
import * as flatbuffers from 'flatbuffers';
export class Person {
    constructor() {
        this.bb = null;
        this.bb_pos = 0;
    }
    __init(i, bb) {
        this.bb_pos = i;
        this.bb = bb;
        return this;
    }
    static getRootAsPerson(bb, obj) {
        return (obj || new Person()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
    }
    static getSizePrefixedRootAsPerson(bb, obj) {
        bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
        return (obj || new Person()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
    }
    name(optionalEncoding) {
        const offset = this.bb.__offset(this.bb_pos, 4);
        return offset ? this.bb.__string(this.bb_pos + offset, optionalEncoding) : null;
    }
    age() {
        const offset = this.bb.__offset(this.bb_pos, 6);
        return offset ? this.bb.readInt16(this.bb_pos + offset) : 0;
    }
    static startPerson(builder) {
        builder.startObject(2);
    }
    static addName(builder, nameOffset) {
        builder.addFieldOffset(0, nameOffset, 0);
    }
    static addAge(builder, age) {
        builder.addFieldInt16(1, age, 0);
    }
    static endPerson(builder) {
        const offset = builder.endObject();
        return offset;
    }
    static finishPersonBuffer(builder, offset) {
        builder.finish(offset);
    }
    static finishSizePrefixedPersonBuffer(builder, offset) {
        builder.finish(offset, undefined, true);
    }
    static createPerson(builder, nameOffset, age) {
        Person.startPerson(builder);
        Person.addName(builder, nameOffset);
        Person.addAge(builder, age);
        return Person.endPerson(builder);
    }
}
