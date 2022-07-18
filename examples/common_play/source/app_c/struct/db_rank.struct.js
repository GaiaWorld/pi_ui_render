_$pi.define("app_c/struct/db_rank.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.ArenaRankDb = exports.RankDb = void 0;
    class RankDb extends struct_mgr_1.Struct {
        static bonType() {
            return RankDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new RankDb();
            o.rid = bb.readInt();
            o.name = bb.readUtf8();
            o.heart = bb.readInt();
            o.startTime = bb.readInt();
            o.show = bb.readUtf8();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.rid);
            bb.writeUtf8(this.name);
            bb.writeInt(this.heart);
            bb.writeInt(this.startTime);
            bb.writeUtf8(this.show);
            return bb;
        }
    }
    exports.RankDb = RankDb;
    RankDb._$info = new sinfo_1.StructInfo("app_c/struct/db_rank.RankDb", 2739416176, null, [new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("name", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("heart", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("startTime", new sinfo_1.EnumType(sinfo_1.Type.Usize), null), new sinfo_1.FieldInfo("show", new sinfo_1.EnumType(sinfo_1.Type.Str), null)]);
    struct_mgr_1.structMgr.register(RankDb._$info.name_hash, RankDb, RankDb._$info.name);
    // 排行列表
    class ArenaRankDb extends struct_mgr_1.Struct {
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return ArenaRankDb._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new ArenaRankDb();
            o.channel = bb.readInt();
            o.list = bb.readArray(() => {
                return o._$EnumTypeMap ? o._$EnumTypeMap(o.list) : RankDb.bonDecode(bb);
            });
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.channel);
            bb.writeArray(this.list, el => {
                el.bonEncode(bb);
            });
            return bb;
        }
    }
    exports.ArenaRankDb = ArenaRankDb;
    ArenaRankDb._$info = new sinfo_1.StructInfo("app_c/struct/db_rank.ArenaRankDb", 4150994781, new Map([["primary", "channel"], ["db", "logfile"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("channel", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("list", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.Struct, RankDb._$info)), null)]);
    struct_mgr_1.structMgr.register(ArenaRankDb._$info.name_hash, ArenaRankDb, ArenaRankDb._$info.name);
});