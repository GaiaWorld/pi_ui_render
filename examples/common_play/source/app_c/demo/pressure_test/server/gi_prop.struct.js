_$pi.define("app_c/demo/pressure_test/server/gi_prop.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.GI_BluePrintFuncRecord = exports.GI_NewWorldFuBenPropRecord = exports.GI_NewWorldPropRecord = exports.GI_UnionSeasonPropRecord = exports.GI_UnionShopPropRecord = exports.GI_JJCPropRecord = exports.GI_NormalPropRecord = void 0;
    class GI_NormalPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.add_15001 = 0;
            //青铜宝箱增加
            this.cost_15001 = 0;
            //青铜宝箱消耗
            this.add_15002 = 0;
            //亮银宝箱增加
            this.cost_15002 = 0;
            //亮银宝箱消耗
            this.add_15003 = 0;
            //黄金宝箱增加
            this.cost_15003 = 0;
            //黄金宝箱消耗
            this.add_18001 = 0;
            //图谱残卷增加
            this.cost_18001 = 0;
            //图谱残卷消耗
            this.add_61056 = 0;
            // 302	熊猫产出精华数量    61056
            this.cost_61056 = 0;
            // 303	玩家消耗的总精华数  61056
            //信物
            this.add_50001 = 0;
            //中原信物    50001
            this.cost_50001 = 0;
            //中原信物  50001
            this.add_50002 = 0;
            //川蜀信物    50002
            this.cost_50002 = 0;
            //川蜀信物  50002
            this.add_50003 = 0;
            //沿海信物    50003
            this.cost_50003 = 0;
            //沿海信物  50003
            this.add_50004 = 0;
            //策划未设计    50004
            this.cost_50004 = 0;
            //策划未设计  50004
            this.add_50005 = 0;
            //策划未设计    50005
            this.cost_50005 = 0;
            //策划未设计  50005
            //稀有材料
            this.add_14001 = 0;
            //红宝石    14001
            this.cost_14001 = 0;
            //红宝石  14001
            this.add_14002 = 0;
            //14002	仙露水
            this.cost_14002 = 0;
            //14002	仙露水
            this.add_14003 = 0;
            //14003	断肠草
            this.cost_14003 = 0;
            //14003	断肠草
            this.add_14004 = 0;
            // 14004	赤血木
            this.cost_14004 = 0;
            // 14004	赤血木
            this.add_14005 = 0;
            //14005	火精石
            this.cost_14005 = 0;
            //14005	火精石
            this.add_14006 = 0;
            // 14006	幻影尘
            this.cost_14006 = 0;
            // 14006	幻影尘
            this.add_14007 = 0;
            //14007	月石
            this.cost_14007 = 0;
            //14007	月石
            this.add_14008 = 0;
            //14008	神龟壳
            this.cost_14008 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_NormalPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_NormalPropRecord();
            o.key = bb.readInt();
            o.add_15001 = bb.readInt();
            o.cost_15001 = bb.readInt();
            o.add_15002 = bb.readInt();
            o.cost_15002 = bb.readInt();
            o.add_15003 = bb.readInt();
            o.cost_15003 = bb.readInt();
            o.add_18001 = bb.readInt();
            o.cost_18001 = bb.readInt();
            o.add_61056 = bb.readInt();
            o.cost_61056 = bb.readInt();
            o.add_50001 = bb.readInt();
            o.cost_50001 = bb.readInt();
            o.add_50002 = bb.readInt();
            o.cost_50002 = bb.readInt();
            o.add_50003 = bb.readInt();
            o.cost_50003 = bb.readInt();
            o.add_50004 = bb.readInt();
            o.cost_50004 = bb.readInt();
            o.add_50005 = bb.readInt();
            o.cost_50005 = bb.readInt();
            o.add_14001 = bb.readInt();
            o.cost_14001 = bb.readInt();
            o.add_14002 = bb.readInt();
            o.cost_14002 = bb.readInt();
            o.add_14003 = bb.readInt();
            o.cost_14003 = bb.readInt();
            o.add_14004 = bb.readInt();
            o.cost_14004 = bb.readInt();
            o.add_14005 = bb.readInt();
            o.cost_14005 = bb.readInt();
            o.add_14006 = bb.readInt();
            o.cost_14006 = bb.readInt();
            o.add_14007 = bb.readInt();
            o.cost_14007 = bb.readInt();
            o.add_14008 = bb.readInt();
            o.cost_14008 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.add_15001);
            bb.writeInt(this.cost_15001);
            bb.writeInt(this.add_15002);
            bb.writeInt(this.cost_15002);
            bb.writeInt(this.add_15003);
            bb.writeInt(this.cost_15003);
            bb.writeInt(this.add_18001);
            bb.writeInt(this.cost_18001);
            bb.writeInt(this.add_61056);
            bb.writeInt(this.cost_61056);
            bb.writeInt(this.add_50001);
            bb.writeInt(this.cost_50001);
            bb.writeInt(this.add_50002);
            bb.writeInt(this.cost_50002);
            bb.writeInt(this.add_50003);
            bb.writeInt(this.cost_50003);
            bb.writeInt(this.add_50004);
            bb.writeInt(this.cost_50004);
            bb.writeInt(this.add_50005);
            bb.writeInt(this.cost_50005);
            bb.writeInt(this.add_14001);
            bb.writeInt(this.cost_14001);
            bb.writeInt(this.add_14002);
            bb.writeInt(this.cost_14002);
            bb.writeInt(this.add_14003);
            bb.writeInt(this.cost_14003);
            bb.writeInt(this.add_14004);
            bb.writeInt(this.cost_14004);
            bb.writeInt(this.add_14005);
            bb.writeInt(this.cost_14005);
            bb.writeInt(this.add_14006);
            bb.writeInt(this.cost_14006);
            bb.writeInt(this.add_14007);
            bb.writeInt(this.cost_14007);
            bb.writeInt(this.add_14008);
            bb.writeInt(this.cost_14008);
            return bb;
        }
    }
    exports.GI_NormalPropRecord = GI_NormalPropRecord;
    GI_NormalPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_NormalPropRecord", 2937655186, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("add_15001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_15001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_15002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_15002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_15003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_15003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_18001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_18001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_61056", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_61056", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_50001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_50001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_50002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_50002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_50003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_50003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_50004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_50004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_50005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_50005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_14008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_14008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_NormalPropRecord._$info.name_hash, GI_NormalPropRecord, GI_NormalPropRecord._$info.name);
    //竞技场兑换的道具统计//[61015,61016,90041,61017,10019,61033,17001,17002,61036,17003,61032,90048,90040,90049,90042,21044] 
    class GI_JJCPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.jjc_add_61015 = 0;
            //大冰石
            this.jjc_add_61016 = 0;
            //极寒冰石
            this.jjc_add_90041 = 0;
            //大还丹
            this.jjc_add_61017 = 0;
            //上古冰石
            this.jjc_add_10019 = 0;
            //铜令牌
            this.jjc_add_61033 = 0;
            //伏羲八卦图
            this.jjc_add_17001 = 0;
            //青铜钥匙
            this.jjc_add_17002 = 0;
            //亮银钥匙
            this.jjc_add_61036 = 0;
            //人参
            this.jjc_add_17003 = 0;
            //黄金钥匙
            this.jjc_add_61032 = 0;
            //人体经脉图
            this.jjc_add_90048 = 0;
            //无常丹
            this.jjc_add_90040 = 0;
            //极品小还丹
            this.jjc_add_90049 = 0;
            //通犀地龙丸
            this.jjc_add_90042 = 0;
            //极品大还丹
            this.jjc_add_21044 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_JJCPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_JJCPropRecord();
            o.key = bb.readInt();
            o.jjc_add_61015 = bb.readInt();
            o.jjc_add_61016 = bb.readInt();
            o.jjc_add_90041 = bb.readInt();
            o.jjc_add_61017 = bb.readInt();
            o.jjc_add_10019 = bb.readInt();
            o.jjc_add_61033 = bb.readInt();
            o.jjc_add_17001 = bb.readInt();
            o.jjc_add_17002 = bb.readInt();
            o.jjc_add_61036 = bb.readInt();
            o.jjc_add_17003 = bb.readInt();
            o.jjc_add_61032 = bb.readInt();
            o.jjc_add_90048 = bb.readInt();
            o.jjc_add_90040 = bb.readInt();
            o.jjc_add_90049 = bb.readInt();
            o.jjc_add_90042 = bb.readInt();
            o.jjc_add_21044 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.jjc_add_61015);
            bb.writeInt(this.jjc_add_61016);
            bb.writeInt(this.jjc_add_90041);
            bb.writeInt(this.jjc_add_61017);
            bb.writeInt(this.jjc_add_10019);
            bb.writeInt(this.jjc_add_61033);
            bb.writeInt(this.jjc_add_17001);
            bb.writeInt(this.jjc_add_17002);
            bb.writeInt(this.jjc_add_61036);
            bb.writeInt(this.jjc_add_17003);
            bb.writeInt(this.jjc_add_61032);
            bb.writeInt(this.jjc_add_90048);
            bb.writeInt(this.jjc_add_90040);
            bb.writeInt(this.jjc_add_90049);
            bb.writeInt(this.jjc_add_90042);
            bb.writeInt(this.jjc_add_21044);
            return bb;
        }
    }
    exports.GI_JJCPropRecord = GI_JJCPropRecord;
    GI_JJCPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_JJCPropRecord", 1979347638, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("jjc_add_61015", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_61016", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_90041", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_61017", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_10019", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_61033", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_17001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_17002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_61036", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_17003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_61032", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_90048", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_90040", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_90049", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_90042", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_add_21044", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_JJCPropRecord._$info.name_hash, GI_JJCPropRecord, GI_JJCPropRecord._$info.name);
    //联盟商店：兑换的道具统计//[15001,90041,21000,21001,21002,90042,61014,21044,15002,15003,61015,21012,21013,21014]
    class GI_UnionShopPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.union_shop_add_15001 = 0;
            //铜箱子
            this.union_shop_add_15002 = 0;
            //银箱子
            this.union_shop_add_15003 = 0;
            //金箱子
            this.union_shop_add_90041 = 0;
            //大还丹
            this.union_shop_add_21000 = 0;
            //黍米黄酒（浑浊）
            this.union_shop_add_21001 = 0;
            //麦曲黄酒（浑浊）
            this.union_shop_add_21002 = 0;
            //糯米黄酒（浑浊） 
            this.union_shop_add_90042 = 0;
            //极品大还丹
            this.union_shop_add_61014 = 0;
            //小冰石
            this.union_shop_add_61015 = 0;
            //大冰石
            this.union_shop_add_21044 = 0;
            //麦曲
            this.union_shop_add_21012 = 0;
            //黍米黄酒（大曲）
            this.union_shop_add_21013 = 0;
            //麦曲黄酒（大曲）
            this.union_shop_add_21014 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_UnionShopPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_UnionShopPropRecord();
            o.key = bb.readInt();
            o.union_shop_add_15001 = bb.readInt();
            o.union_shop_add_15002 = bb.readInt();
            o.union_shop_add_15003 = bb.readInt();
            o.union_shop_add_90041 = bb.readInt();
            o.union_shop_add_21000 = bb.readInt();
            o.union_shop_add_21001 = bb.readInt();
            o.union_shop_add_21002 = bb.readInt();
            o.union_shop_add_90042 = bb.readInt();
            o.union_shop_add_61014 = bb.readInt();
            o.union_shop_add_61015 = bb.readInt();
            o.union_shop_add_21044 = bb.readInt();
            o.union_shop_add_21012 = bb.readInt();
            o.union_shop_add_21013 = bb.readInt();
            o.union_shop_add_21014 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.union_shop_add_15001);
            bb.writeInt(this.union_shop_add_15002);
            bb.writeInt(this.union_shop_add_15003);
            bb.writeInt(this.union_shop_add_90041);
            bb.writeInt(this.union_shop_add_21000);
            bb.writeInt(this.union_shop_add_21001);
            bb.writeInt(this.union_shop_add_21002);
            bb.writeInt(this.union_shop_add_90042);
            bb.writeInt(this.union_shop_add_61014);
            bb.writeInt(this.union_shop_add_61015);
            bb.writeInt(this.union_shop_add_21044);
            bb.writeInt(this.union_shop_add_21012);
            bb.writeInt(this.union_shop_add_21013);
            bb.writeInt(this.union_shop_add_21014);
            return bb;
        }
    }
    exports.GI_UnionShopPropRecord = GI_UnionShopPropRecord;
    GI_UnionShopPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_UnionShopPropRecord", 3745085441, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("union_shop_add_15001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_15002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_15003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_90041", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21000", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_90042", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_61014", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_61015", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21044", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21012", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21013", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_shop_add_21014", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_UnionShopPropRecord._$info.name_hash, GI_UnionShopPropRecord, GI_UnionShopPropRecord._$info.name);
    //联盟赛季：兑换的道具统计//[10017,21030,21031,21032,10016,10019,17001,17002,17003,90042,21024,21025,21026]
    class GI_UnionSeasonPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.union_season_add_10016 = 0;
            //江湖令牌
            this.union_season_add_10017 = 0;
            //洞府令牌
            this.union_season_add_10019 = 0;
            //铜令牌
            this.union_season_add_21030 = 0;
            //黍米黄酒（琼浆）
            this.union_season_add_21031 = 0;
            //麦曲黄酒（琼浆）
            this.union_season_add_21032 = 0;
            //糯米黄酒（琼浆）
            this.union_season_add_17001 = 0;
            //青铜钥匙
            this.union_season_add_17002 = 0;
            //亮银钥匙
            this.union_season_add_17003 = 0;
            //黄金钥匙
            this.union_season_add_90042 = 0;
            //极品大还丹
            this.union_season_add_21024 = 0;
            //黍米黄酒（佳酿）
            this.union_season_add_21025 = 0;
            //麦曲黄酒（佳酿）
            this.union_season_add_21026 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_UnionSeasonPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_UnionSeasonPropRecord();
            o.key = bb.readInt();
            o.union_season_add_10016 = bb.readInt();
            o.union_season_add_10017 = bb.readInt();
            o.union_season_add_10019 = bb.readInt();
            o.union_season_add_21030 = bb.readInt();
            o.union_season_add_21031 = bb.readInt();
            o.union_season_add_21032 = bb.readInt();
            o.union_season_add_17001 = bb.readInt();
            o.union_season_add_17002 = bb.readInt();
            o.union_season_add_17003 = bb.readInt();
            o.union_season_add_90042 = bb.readInt();
            o.union_season_add_21024 = bb.readInt();
            o.union_season_add_21025 = bb.readInt();
            o.union_season_add_21026 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.union_season_add_10016);
            bb.writeInt(this.union_season_add_10017);
            bb.writeInt(this.union_season_add_10019);
            bb.writeInt(this.union_season_add_21030);
            bb.writeInt(this.union_season_add_21031);
            bb.writeInt(this.union_season_add_21032);
            bb.writeInt(this.union_season_add_17001);
            bb.writeInt(this.union_season_add_17002);
            bb.writeInt(this.union_season_add_17003);
            bb.writeInt(this.union_season_add_90042);
            bb.writeInt(this.union_season_add_21024);
            bb.writeInt(this.union_season_add_21025);
            bb.writeInt(this.union_season_add_21026);
            return bb;
        }
    }
    exports.GI_UnionSeasonPropRecord = GI_UnionSeasonPropRecord;
    GI_UnionSeasonPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_UnionSeasonPropRecord", 1194634700, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("union_season_add_10016", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_10017", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_10019", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21030", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21031", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21032", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_17001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_17002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_17003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_90042", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21024", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21025", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("union_season_add_21026", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_UnionSeasonPropRecord._$info.name_hash, GI_UnionSeasonPropRecord, GI_UnionSeasonPropRecord._$info.name);
    //武林投放道具//[15001,15002,15003,17000,16000,16001,16002,16003,16004,16005,16006,16007,16008,16009,16010,16011,16012,15004,15005,15006,15007,61001,61002,61003,61006,61007,61008,61009,61053,61054]
    class GI_NewWorldPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.world_add_15001 = 0;
            //青铜宝箱    
            this.world_add_15002 = 0;
            //亮银宝箱
            this.world_add_15003 = 0;
            //黄金宝箱
            this.world_add_17000 = 0;
            //宝物袋
            this.world_add_16000 = 0;
            //2级宝袋
            this.world_add_16001 = 0;
            //3级宝袋
            this.world_add_16002 = 0;
            //4级宝袋
            this.world_add_16003 = 0;
            //5级宝袋
            this.world_add_16004 = 0;
            //6级宝袋
            this.world_add_16005 = 0;
            //7级宝袋
            this.world_add_16006 = 0;
            //8级宝袋
            this.world_add_16007 = 0;
            //9级宝袋
            this.world_add_16008 = 0;
            //10级宝袋
            this.world_add_16009 = 0;
            //11级宝袋
            this.world_add_16010 = 0;
            //12级宝袋
            this.world_add_15004 = 0;
            //闯王宝箱
            this.world_add_15005 = 0;
            //古城宝箱
            this.world_add_15006 = 0;
            //龙脉宝箱
            this.world_add_15007 = 0;
            //奇缘宝箱
            this.world_add_16011 = 0;
            //13级宝袋
            this.world_add_16012 = 0;
            //宝物袋
            this.world_add_61001 = 0;
            //一箱酒
            this.world_add_61002 = 0;
            //一箱好酒
            this.world_add_61003 = 0;
            //令牌道具包
            this.world_add_61007 = 0;
            //初级丹药包
            this.world_add_61008 = 0;
            //中级丹药包
            this.world_add_61009 = 0;
            //高级丹药包
            this.world_add_61006 = 0;
            //图纸碎片包
            this.world_add_61053 = 0;
            //初级材料包
            this.world_add_61054 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_NewWorldPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_NewWorldPropRecord();
            o.key = bb.readInt();
            o.world_add_15001 = bb.readInt();
            o.world_add_15002 = bb.readInt();
            o.world_add_15003 = bb.readInt();
            o.world_add_17000 = bb.readInt();
            o.world_add_16000 = bb.readInt();
            o.world_add_16001 = bb.readInt();
            o.world_add_16002 = bb.readInt();
            o.world_add_16003 = bb.readInt();
            o.world_add_16004 = bb.readInt();
            o.world_add_16005 = bb.readInt();
            o.world_add_16006 = bb.readInt();
            o.world_add_16007 = bb.readInt();
            o.world_add_16008 = bb.readInt();
            o.world_add_16009 = bb.readInt();
            o.world_add_16010 = bb.readInt();
            o.world_add_15004 = bb.readInt();
            o.world_add_15005 = bb.readInt();
            o.world_add_15006 = bb.readInt();
            o.world_add_15007 = bb.readInt();
            o.world_add_16011 = bb.readInt();
            o.world_add_16012 = bb.readInt();
            o.world_add_61001 = bb.readInt();
            o.world_add_61002 = bb.readInt();
            o.world_add_61003 = bb.readInt();
            o.world_add_61007 = bb.readInt();
            o.world_add_61008 = bb.readInt();
            o.world_add_61009 = bb.readInt();
            o.world_add_61006 = bb.readInt();
            o.world_add_61053 = bb.readInt();
            o.world_add_61054 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.world_add_15001);
            bb.writeInt(this.world_add_15002);
            bb.writeInt(this.world_add_15003);
            bb.writeInt(this.world_add_17000);
            bb.writeInt(this.world_add_16000);
            bb.writeInt(this.world_add_16001);
            bb.writeInt(this.world_add_16002);
            bb.writeInt(this.world_add_16003);
            bb.writeInt(this.world_add_16004);
            bb.writeInt(this.world_add_16005);
            bb.writeInt(this.world_add_16006);
            bb.writeInt(this.world_add_16007);
            bb.writeInt(this.world_add_16008);
            bb.writeInt(this.world_add_16009);
            bb.writeInt(this.world_add_16010);
            bb.writeInt(this.world_add_15004);
            bb.writeInt(this.world_add_15005);
            bb.writeInt(this.world_add_15006);
            bb.writeInt(this.world_add_15007);
            bb.writeInt(this.world_add_16011);
            bb.writeInt(this.world_add_16012);
            bb.writeInt(this.world_add_61001);
            bb.writeInt(this.world_add_61002);
            bb.writeInt(this.world_add_61003);
            bb.writeInt(this.world_add_61007);
            bb.writeInt(this.world_add_61008);
            bb.writeInt(this.world_add_61009);
            bb.writeInt(this.world_add_61006);
            bb.writeInt(this.world_add_61053);
            bb.writeInt(this.world_add_61054);
            return bb;
        }
    }
    exports.GI_NewWorldPropRecord = GI_NewWorldPropRecord;
    GI_NewWorldPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_NewWorldPropRecord", 2623820459, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("world_add_15001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_17000", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16000", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16010", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_15007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16011", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_16012", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61053", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_add_61054", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_NewWorldPropRecord._$info.name_hash, GI_NewWorldPropRecord, GI_NewWorldPropRecord._$info.name);
    //武林副本投放道具//[15001,15002,15003,17000,16000,16001,16002,16003,16004,16005,16006,16007,16008,16009,16010,16011,16012,15004,15005,15006,15007,61001,61002,61003,61006,61007,61008,61009,61053,61054]
    class GI_NewWorldFuBenPropRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.world_fb_add_15001 = 0;
            //青铜宝箱    
            this.world_fb_add_15002 = 0;
            //亮银宝箱
            this.world_fb_add_15003 = 0;
            //黄金宝箱
            this.world_fb_add_17000 = 0;
            //宝物袋
            this.world_fb_add_16000 = 0;
            //2级宝袋
            this.world_fb_add_16001 = 0;
            //3级宝袋
            this.world_fb_add_16002 = 0;
            //4级宝袋
            this.world_fb_add_16003 = 0;
            //5级宝袋
            this.world_fb_add_16004 = 0;
            //6级宝袋
            this.world_fb_add_16005 = 0;
            //7级宝袋
            this.world_fb_add_16006 = 0;
            //8级宝袋
            this.world_fb_add_16007 = 0;
            //9级宝袋
            this.world_fb_add_16008 = 0;
            //10级宝袋
            this.world_fb_add_16009 = 0;
            //11级宝袋
            this.world_fb_add_16010 = 0;
            //12级宝袋
            this.world_fb_add_15004 = 0;
            //闯王宝箱
            this.world_fb_add_15005 = 0;
            //古城宝箱
            this.world_fb_add_15006 = 0;
            //龙脉宝箱
            this.world_fb_add_15007 = 0;
            //奇缘宝箱
            this.world_fb_add_16011 = 0;
            //13级宝袋
            this.world_fb_add_16012 = 0;
            //宝物袋
            this.world_fb_add_61001 = 0;
            //一箱酒
            this.world_fb_add_61002 = 0;
            //一箱好酒
            this.world_fb_add_61003 = 0;
            //令牌道具包
            this.world_fb_add_61007 = 0;
            //初级丹药包
            this.world_fb_add_61008 = 0;
            //中级丹药包
            this.world_fb_add_61009 = 0;
            //高级丹药包
            this.world_fb_add_61006 = 0;
            //图纸碎片包
            this.world_fb_add_61053 = 0;
            //初级材料包
            this.world_fb_add_61054 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_NewWorldFuBenPropRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_NewWorldFuBenPropRecord();
            o.key = bb.readInt();
            o.world_fb_add_15001 = bb.readInt();
            o.world_fb_add_15002 = bb.readInt();
            o.world_fb_add_15003 = bb.readInt();
            o.world_fb_add_17000 = bb.readInt();
            o.world_fb_add_16000 = bb.readInt();
            o.world_fb_add_16001 = bb.readInt();
            o.world_fb_add_16002 = bb.readInt();
            o.world_fb_add_16003 = bb.readInt();
            o.world_fb_add_16004 = bb.readInt();
            o.world_fb_add_16005 = bb.readInt();
            o.world_fb_add_16006 = bb.readInt();
            o.world_fb_add_16007 = bb.readInt();
            o.world_fb_add_16008 = bb.readInt();
            o.world_fb_add_16009 = bb.readInt();
            o.world_fb_add_16010 = bb.readInt();
            o.world_fb_add_15004 = bb.readInt();
            o.world_fb_add_15005 = bb.readInt();
            o.world_fb_add_15006 = bb.readInt();
            o.world_fb_add_15007 = bb.readInt();
            o.world_fb_add_16011 = bb.readInt();
            o.world_fb_add_16012 = bb.readInt();
            o.world_fb_add_61001 = bb.readInt();
            o.world_fb_add_61002 = bb.readInt();
            o.world_fb_add_61003 = bb.readInt();
            o.world_fb_add_61007 = bb.readInt();
            o.world_fb_add_61008 = bb.readInt();
            o.world_fb_add_61009 = bb.readInt();
            o.world_fb_add_61006 = bb.readInt();
            o.world_fb_add_61053 = bb.readInt();
            o.world_fb_add_61054 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.world_fb_add_15001);
            bb.writeInt(this.world_fb_add_15002);
            bb.writeInt(this.world_fb_add_15003);
            bb.writeInt(this.world_fb_add_17000);
            bb.writeInt(this.world_fb_add_16000);
            bb.writeInt(this.world_fb_add_16001);
            bb.writeInt(this.world_fb_add_16002);
            bb.writeInt(this.world_fb_add_16003);
            bb.writeInt(this.world_fb_add_16004);
            bb.writeInt(this.world_fb_add_16005);
            bb.writeInt(this.world_fb_add_16006);
            bb.writeInt(this.world_fb_add_16007);
            bb.writeInt(this.world_fb_add_16008);
            bb.writeInt(this.world_fb_add_16009);
            bb.writeInt(this.world_fb_add_16010);
            bb.writeInt(this.world_fb_add_15004);
            bb.writeInt(this.world_fb_add_15005);
            bb.writeInt(this.world_fb_add_15006);
            bb.writeInt(this.world_fb_add_15007);
            bb.writeInt(this.world_fb_add_16011);
            bb.writeInt(this.world_fb_add_16012);
            bb.writeInt(this.world_fb_add_61001);
            bb.writeInt(this.world_fb_add_61002);
            bb.writeInt(this.world_fb_add_61003);
            bb.writeInt(this.world_fb_add_61007);
            bb.writeInt(this.world_fb_add_61008);
            bb.writeInt(this.world_fb_add_61009);
            bb.writeInt(this.world_fb_add_61006);
            bb.writeInt(this.world_fb_add_61053);
            bb.writeInt(this.world_fb_add_61054);
            return bb;
        }
    }
    exports.GI_NewWorldFuBenPropRecord = GI_NewWorldFuBenPropRecord;
    GI_NewWorldFuBenPropRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_NewWorldFuBenPropRecord", 4044887337, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("world_fb_add_15001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_17000", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16000", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16010", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15005", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_15007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16011", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_16012", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61053", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("world_fb_add_61054", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_NewWorldFuBenPropRecord._$info.name_hash, GI_NewWorldFuBenPropRecord, GI_NewWorldFuBenPropRecord._$info.name);
    //图纸功能监听(玩家获得图纸)(此表数据是累加值)//等级  1 、 5 、10 、15、 20 、25、 30、 35 、40 、45、 50、 55、 60、 65、 70、75、80、85、90、95、100
    class GI_BluePrintFuncRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.blue_level_1 = 0;
            //1级图纸    
            this.blue_level_5 = 0;
            //5级图纸    
            this.blue_level_10 = 0;
            //10级图纸    
            this.blue_level_15 = 0;
            //15级图纸    
            this.blue_level_20 = 0;
            //20级图纸    
            this.blue_level_25 = 0;
            //25级图纸    
            this.blue_level_30 = 0;
            //30级图纸    
            this.blue_level_35 = 0;
            //35级图纸    
            this.blue_level_40 = 0;
            //40级图纸    
            this.blue_level_45 = 0;
            //45级图纸    
            this.blue_level_50 = 0;
            //50级图纸    
            this.blue_level_55 = 0;
            //55级图纸    
            this.blue_level_60 = 0;
            //60级图纸    
            this.blue_level_65 = 0;
            //65级图纸    
            this.blue_level_70 = 0;
            //70级图纸    
            this.blue_level_75 = 0;
            //75级图纸    
            this.blue_level_80 = 0;
            //80级图纸    
            this.blue_level_85 = 0;
            //85级图纸    
            this.blue_level_90 = 0;
            //90级图纸    
            this.blue_level_95 = 0;
            //95级图纸    
            this.blue_level_100 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_BluePrintFuncRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_BluePrintFuncRecord();
            o.key = bb.readInt();
            o.blue_level_1 = bb.readInt();
            o.blue_level_5 = bb.readInt();
            o.blue_level_10 = bb.readInt();
            o.blue_level_15 = bb.readInt();
            o.blue_level_20 = bb.readInt();
            o.blue_level_25 = bb.readInt();
            o.blue_level_30 = bb.readInt();
            o.blue_level_35 = bb.readInt();
            o.blue_level_40 = bb.readInt();
            o.blue_level_45 = bb.readInt();
            o.blue_level_50 = bb.readInt();
            o.blue_level_55 = bb.readInt();
            o.blue_level_60 = bb.readInt();
            o.blue_level_65 = bb.readInt();
            o.blue_level_70 = bb.readInt();
            o.blue_level_75 = bb.readInt();
            o.blue_level_80 = bb.readInt();
            o.blue_level_85 = bb.readInt();
            o.blue_level_90 = bb.readInt();
            o.blue_level_95 = bb.readInt();
            o.blue_level_100 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.blue_level_1);
            bb.writeInt(this.blue_level_5);
            bb.writeInt(this.blue_level_10);
            bb.writeInt(this.blue_level_15);
            bb.writeInt(this.blue_level_20);
            bb.writeInt(this.blue_level_25);
            bb.writeInt(this.blue_level_30);
            bb.writeInt(this.blue_level_35);
            bb.writeInt(this.blue_level_40);
            bb.writeInt(this.blue_level_45);
            bb.writeInt(this.blue_level_50);
            bb.writeInt(this.blue_level_55);
            bb.writeInt(this.blue_level_60);
            bb.writeInt(this.blue_level_65);
            bb.writeInt(this.blue_level_70);
            bb.writeInt(this.blue_level_75);
            bb.writeInt(this.blue_level_80);
            bb.writeInt(this.blue_level_85);
            bb.writeInt(this.blue_level_90);
            bb.writeInt(this.blue_level_95);
            bb.writeInt(this.blue_level_100);
            return bb;
        }
    }
    exports.GI_BluePrintFuncRecord = GI_BluePrintFuncRecord;
    GI_BluePrintFuncRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_prop.GI_BluePrintFuncRecord", 563514270, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("blue_level_1", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_5", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_10", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_15", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_20", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_25", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_30", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_35", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_40", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_45", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_50", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_55", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_60", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_65", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_70", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_75", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_80", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_85", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_90", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_95", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("blue_level_100", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_BluePrintFuncRecord._$info.name_hash, GI_BluePrintFuncRecord, GI_BluePrintFuncRecord._$info.name);
});