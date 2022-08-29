_$pi.define("app_c/fight/ecs/round_skill/buffModel.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.BuffModel = void 0;
    class BuffModel extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return BuffModel._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new BuffModel();
            o.id = bb.readInt();
            o.name = bb.readUtf8();
            o.probility = bb.readInt();
            o.type_group = bb.readUtf8();
            o.immunity = bb.readInt();
            o.is_cover = bb.readUtf8();
            o.coverGroup = bb.readInt();
            o.fireType = bb.readUtf8();
            o.fireCondition = bb.readUtf8();
            o.cooldown = bb.readInt();
            o.fireMaxTimes = bb.readInt();
            o.upEffectFunc = bb.readUtf8();
            o.upEffectType = bb.readUtf8();
            o.upEffectArgs = bb.readInt();
            o.downEffectFunc = bb.readUtf8();
            o.downEffectType = bb.readUtf8();
            o.downEffectArgs = bb.readInt();
            o.fireEffectFunc = bb.readUtf8();
            o.fireEffectType = bb.readUtf8();
            o.fireEffectArgs = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.id);
            bb.writeUtf8(this.name);
            bb.writeInt(this.probility);
            bb.writeUtf8(this.type_group);
            bb.writeInt(this.immunity);
            bb.writeUtf8(this.is_cover);
            bb.writeInt(this.coverGroup);
            bb.writeUtf8(this.fireType);
            bb.writeUtf8(this.fireCondition);
            bb.writeInt(this.cooldown);
            bb.writeInt(this.fireMaxTimes);
            bb.writeUtf8(this.upEffectFunc);
            bb.writeUtf8(this.upEffectType);
            bb.writeInt(this.upEffectArgs);
            bb.writeUtf8(this.downEffectFunc);
            bb.writeUtf8(this.downEffectType);
            bb.writeInt(this.downEffectArgs);
            bb.writeUtf8(this.fireEffectFunc);
            bb.writeUtf8(this.fireEffectType);
            bb.writeInt(this.fireEffectArgs);
            return bb;
        }
    }
    exports.BuffModel = BuffModel;
    BuffModel._$info = new sinfo_1.StructInfo("app_c/fight/ecs/round_skill/buffModel.BuffModel", 3713332565, new Map([["db", "memory"], ["primary", "id"]]), [new sinfo_1.FieldInfo("id", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("name", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("probility", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("type_group", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("immunity", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("is_cover", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("coverGroup", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("fireType", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("fireCondition", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("cooldown", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("fireMaxTimes", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("upEffectFunc", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("upEffectType", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("upEffectArgs", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("downEffectFunc", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("downEffectType", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("downEffectArgs", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("fireEffectFunc", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("fireEffectType", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("fireEffectArgs", new sinfo_1.EnumType(sinfo_1.Type.I32), null)]);
    struct_mgr_1.structMgr.register(BuffModel._$info.name_hash, BuffModel, BuffModel._$info.name);
    BuffModel.prototype.__create = function (o) {
        let r = new BuffModel();
        r.__ = o;
        r.id = o[0];
        return r;
    };
    BuffModel.prototype.__init = function () {
        this.name = this.__[1];
        this.probility = this.__[2];
        this.type_group = this.__[3];
        this.immunity = this.__[4];
        this.is_cover = this.__[5];
        this.coverGroup = this.__[6];
        this.fireType = this.__[7];
        this.fireCondition = this.__[8];
        this.cooldown = this.__[9];
        this.fireMaxTimes = this.__[10];
        this.upEffectFunc = this.__[11];
        this.upEffectType = this.__[12];
        this.upEffectArgs = this.__[13];
        this.downEffectFunc = this.__[14];
        this.downEffectType = this.__[15];
        this.downEffectArgs = this.__[16];
        this.fireEffectFunc = this.__[17];
        this.fireEffectType = this.__[18];
        this.fireEffectArgs = this.__[19];
        delete this.__;
        return this;
    };
});