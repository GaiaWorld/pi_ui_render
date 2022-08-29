_$pi.define("app_c/fight/ecs/round_skill/skillModel.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.SkillModel = void 0;
    class SkillModel extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return SkillModel._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new SkillModel();
            o.id = bb.readInt();
            o.name = bb.readUtf8();
            o.castPreDelay = bb.readInt();
            o.castAfterDelay = bb.readInt();
            o.targetReSelect = bb.readBool();
            o.castBuffID = bb.read();
            o.hitBuffID = bb.read();
            o.beHitBuffID = bb.read();
            o.skillWay = bb.readUtf8();
            o.skillType = bb.readUtf8();
            o.skillGroup = bb.readInt();
            o.targetType = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.id);
            bb.writeUtf8(this.name);
            bb.writeInt(this.castPreDelay);
            bb.writeInt(this.castAfterDelay);
            bb.writeBool(this.targetReSelect);
            bb.write(this.castBuffID);
            bb.write(this.hitBuffID);
            bb.write(this.beHitBuffID);
            bb.writeUtf8(this.skillWay);
            bb.writeUtf8(this.skillType);
            bb.writeInt(this.skillGroup);
            bb.writeUtf8(this.targetType);
            return bb;
        }
    }
    exports.SkillModel = SkillModel;
    SkillModel._$info = new sinfo_1.StructInfo("app_c/fight/ecs/round_skill/skillModel.SkillModel", 2723383730, new Map([["db", "memory"], ["primary", "id"]]), [new sinfo_1.FieldInfo("id", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("name", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("castPreDelay", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("castAfterDelay", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("targetReSelect", new sinfo_1.EnumType(sinfo_1.Type.Bool), null), new sinfo_1.FieldInfo("castBuffID", new sinfo_1.EnumType(sinfo_1.Type.Struct, new sinfo_1.StructInfo("_$Json", 1, null, null)), null), new sinfo_1.FieldInfo("hitBuffID", new sinfo_1.EnumType(sinfo_1.Type.Struct, new sinfo_1.StructInfo("_$Json", 1, null, null)), null), new sinfo_1.FieldInfo("beHitBuffID", new sinfo_1.EnumType(sinfo_1.Type.Struct, new sinfo_1.StructInfo("_$Json", 1, null, null)), null), new sinfo_1.FieldInfo("skillWay", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("skillType", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("skillGroup", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("targetType", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(SkillModel._$info.name_hash, SkillModel, SkillModel._$info.name);
    SkillModel.prototype.__create = function (o) {
        let r = new SkillModel();
        r.__ = o;
        r.id = o[0];
        return r;
    };
    SkillModel.prototype.__init = function () {
        this.name = this.__[1];
        this.castPreDelay = this.__[2];
        this.castAfterDelay = this.__[3];
        this.targetReSelect = this.__[4];
        this.castBuffID = this.__[5];
        this.hitBuffID = this.__[6];
        this.beHitBuffID = this.__[7];
        this.skillWay = this.__[8];
        this.skillType = this.__[9];
        this.skillGroup = this.__[10];
        this.targetType = this.__[11];
        delete this.__;
        return this;
    };
});