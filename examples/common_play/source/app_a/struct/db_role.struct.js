_$pi.define("app_a/struct/db_role.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.IdGeneratorDb = exports.UserMapping = exports.UserInfo = exports.CurrencyDb = exports.GoldDb = exports.RoleDb = void 0;
    class RoleDb extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return RoleDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new RoleDb();
            o.rid = bb.readInt();
            o.plat = bb.readUtf8();
            o.startTime = bb.readInt();
            o.serverId = bb.readInt();
            o.styleId = bb.readInt();
            o.name = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.rid);
            bb.writeUtf8(this.plat);
            bb.writeInt(this.startTime);
            bb.writeInt(this.serverId);
            bb.writeInt(this.styleId);
            bb.writeUtf8(this.name);
            return bb;
        }
    }
    exports.RoleDb = RoleDb;
    RoleDb._$info = new sinfo_1.StructInfo("app_a/struct/db_role.RoleDb", 3517817922, new Map([["primary", "rid"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("plat", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("startTime", new sinfo_1.EnumType(sinfo_1.Type.Usize), null), new sinfo_1.FieldInfo("serverId", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("styleId", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("name", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(RoleDb._$info.name_hash, RoleDb, RoleDb._$info.name);
    // 货币1: 金币
    class GoldDb extends struct_mgr_1.Struct {
        static bonType() {
            return GoldDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GoldDb();
            o.add = bb.readUtf8();
            o.cost = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.add);
            bb.writeUtf8(this.cost);
            return bb;
        }
    }
    exports.GoldDb = GoldDb;
    GoldDb._$info = new sinfo_1.StructInfo("app_a/struct/db_role.GoldDb", 1865741867, null, [new sinfo_1.FieldInfo("add", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("cost", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(GoldDb._$info.name_hash, GoldDb, GoldDb._$info.name);
    //经济
    class CurrencyDb extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            this.gold && this.gold.addMeta(mgr);
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
            this.gold && this.gold.removeMeta();
        }
        static bonType() {
            return CurrencyDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new CurrencyDb();
            o.rid = bb.readInt();
            o.gold = o._$EnumTypeMap ? o._$EnumTypeMap(o.gold) : GoldDb.bonDecode(bb);
            o.heart = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.rid);
            this.gold.bonEncode(bb);
            bb.writeInt(this.heart);
            return bb;
        }
    }
    exports.CurrencyDb = CurrencyDb;
    CurrencyDb._$info = new sinfo_1.StructInfo("app_a/struct/db_role.CurrencyDb", 1387895914, new Map([["primary", "rid"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("gold", new sinfo_1.EnumType(sinfo_1.Type.Struct, GoldDb._$info), null), new sinfo_1.FieldInfo("heart", new sinfo_1.EnumType(sinfo_1.Type.U32), null)]);
    struct_mgr_1.structMgr.register(CurrencyDb._$info.name_hash, CurrencyDb, CurrencyDb._$info.name);
    /**
    *测试复合键
    */
    class UserInfo extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return UserInfo._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new UserInfo();
            o.key = bb.readArray(() => {
                return bb.readInt();
            });
            o.value = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeArray(this.key, el => {
                bb.writeInt(el);
            });
            bb.writeUtf8(this.value);
            return bb;
        }
    }
    exports.UserInfo = UserInfo;
    UserInfo._$info = new sinfo_1.StructInfo("app_a/struct/db_role.UserInfo", 4230356818, new Map([["primary", "key"], ["db", "logfile"], ["dbMonitors", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.Usize)), null), new sinfo_1.FieldInfo("value", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(UserInfo._$info.name_hash, UserInfo, UserInfo._$info.name);
    // 用户映射
    class UserMapping extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return UserMapping._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new UserMapping();
            o.uuid = bb.readUtf8();
            o.rid = bb.readArray(() => {
                return bb.readInt();
            });
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.uuid);
            bb.writeArray(this.rid, el => {
                bb.writeInt(el);
            });
            return bb;
        }
    }
    exports.UserMapping = UserMapping;
    UserMapping._$info = new sinfo_1.StructInfo("app_a/struct/db_role.UserMapping", 2658580023, new Map([["primary", "uuid"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("uuid", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.U32)), null)]);
    struct_mgr_1.structMgr.register(UserMapping._$info.name_hash, UserMapping, UserMapping._$info.name);
    // 自增主键
    class IdGeneratorDb extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return IdGeneratorDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new IdGeneratorDb();
            o.key = bb.readUtf8();
            o.value = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.key);
            bb.writeInt(this.value);
            return bb;
        }
    }
    exports.IdGeneratorDb = IdGeneratorDb;
    IdGeneratorDb._$info = new sinfo_1.StructInfo("app_a/struct/db_role.IdGeneratorDb", 175755073, new Map([["primary", "key"], ["db", "logfile"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("value", new sinfo_1.EnumType(sinfo_1.Type.U32), null)]);
    struct_mgr_1.structMgr.register(IdGeneratorDb._$info.name_hash, IdGeneratorDb, IdGeneratorDb._$info.name);
});