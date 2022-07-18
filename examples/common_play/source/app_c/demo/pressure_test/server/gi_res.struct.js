_$pi.define("app_c/demo/pressure_test/server/gi_res.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.GI_JinCost = exports.GI_JinAdd = exports.GI_RechargeRMB = exports.GI_DiamondAdd = exports.GI_DiamondCost = exports.GI_MoneyAdd = exports.GI_MoneyCost = exports.GI_ChangeCoinRecord = void 0;
    class GI_ChangeCoinRecord extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.add_10001 = 0;
            //木头
            this.add_10002 = 0;
            //铁块
            this.add_10003 = 0;
            //皮革
            this.add_10004 = 0;
            //草药
            this.add_11001 = 0;
            //获得的铜币
            this.add_11002 = 0;
            //获得的银元宝    
            this.add_11003 = 0;
            //获得的金元宝
            this.add_10014 = 0;
            //获得的竞技场积分
            this.add_12001 = 0;
            //获得的悟性点
            this.add_10011 = 0;
            //获得的体力值
            this.add_11006 = 0;
            //个人联盟贡献值
            this.add_11007 = 0;
            //联盟攻伐勋章
            this.add_11008 = 0;
            //联盟荣耀勋章
            this.add_11009 = 0;
            //联盟传奇勋章
            this.cost_10001 = 0;
            //木头
            this.cost_10002 = 0;
            //铁块
            this.cost_10003 = 0;
            //皮革
            this.cost_10004 = 0;
            //草药
            this.cost_11001 = 0;
            //消耗的铜币
            this.cost_11002 = 0;
            //消耗的银元宝
            this.cost_11003 = 0;
            //消耗的金元宝
            this.cost_10014 = 0;
            //消耗的竞技场积分
            this.cost_12001 = 0;
            //消耗的悟性点
            this.cost_10011 = 0;
            //消耗的体力值
            this.cost_11006 = 0;
            //消耗个人联盟贡献值
            this.cost_11007 = 0;
            //消耗联盟攻伐勋章
            this.cost_11008 = 0;
            //消耗联盟荣耀勋章
            this.cost_11009 = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_ChangeCoinRecord._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_ChangeCoinRecord();
            o.key = bb.readInt();
            o.add_10001 = bb.readInt();
            o.add_10002 = bb.readInt();
            o.add_10003 = bb.readInt();
            o.add_10004 = bb.readInt();
            o.add_11001 = bb.readInt();
            o.add_11002 = bb.readInt();
            o.add_11003 = bb.readInt();
            o.add_10014 = bb.readInt();
            o.add_12001 = bb.readInt();
            o.add_10011 = bb.readInt();
            o.add_11006 = bb.readInt();
            o.add_11007 = bb.readInt();
            o.add_11008 = bb.readInt();
            o.add_11009 = bb.readInt();
            o.cost_10001 = bb.readInt();
            o.cost_10002 = bb.readInt();
            o.cost_10003 = bb.readInt();
            o.cost_10004 = bb.readInt();
            o.cost_11001 = bb.readInt();
            o.cost_11002 = bb.readInt();
            o.cost_11003 = bb.readInt();
            o.cost_10014 = bb.readInt();
            o.cost_12001 = bb.readInt();
            o.cost_10011 = bb.readInt();
            o.cost_11006 = bb.readInt();
            o.cost_11007 = bb.readInt();
            o.cost_11008 = bb.readInt();
            o.cost_11009 = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.add_10001);
            bb.writeInt(this.add_10002);
            bb.writeInt(this.add_10003);
            bb.writeInt(this.add_10004);
            bb.writeInt(this.add_11001);
            bb.writeInt(this.add_11002);
            bb.writeInt(this.add_11003);
            bb.writeInt(this.add_10014);
            bb.writeInt(this.add_12001);
            bb.writeInt(this.add_10011);
            bb.writeInt(this.add_11006);
            bb.writeInt(this.add_11007);
            bb.writeInt(this.add_11008);
            bb.writeInt(this.add_11009);
            bb.writeInt(this.cost_10001);
            bb.writeInt(this.cost_10002);
            bb.writeInt(this.cost_10003);
            bb.writeInt(this.cost_10004);
            bb.writeInt(this.cost_11001);
            bb.writeInt(this.cost_11002);
            bb.writeInt(this.cost_11003);
            bb.writeInt(this.cost_10014);
            bb.writeInt(this.cost_12001);
            bb.writeInt(this.cost_10011);
            bb.writeInt(this.cost_11006);
            bb.writeInt(this.cost_11007);
            bb.writeInt(this.cost_11008);
            bb.writeInt(this.cost_11009);
            return bb;
        }
    }
    exports.GI_ChangeCoinRecord = GI_ChangeCoinRecord;
    GI_ChangeCoinRecord._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_ChangeCoinRecord", 177033796, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("add_10001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_10002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_10003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_10004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_10014", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_12001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_10011", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("add_11009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10004", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11002", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11003", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10014", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_12001", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_10011", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11006", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11007", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11008", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("cost_11009", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_ChangeCoinRecord._$info.name_hash, GI_ChangeCoinRecord, GI_ChangeCoinRecord._$info.name);
    //资源变化表(铜币消耗)(改变表)//["upBuild","bodyStudy","trade","book","unionDonate","worldShop"]
    class GI_MoneyCost extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.upBuild_costMoney = 0;
            // 353	来自于建筑升级的消耗	每天总量	积累值
            this.bodyStudy_costMoney = 0;
            // 354	来自于修炼的消耗	每天总量	积累值
            this.trade_costMoney = 0;
            // 355	来自于交易所的消耗	每天总量	积累值
            this.book_costMoney = 0;
            // 356	来自于炼丹的消耗	每天总量	积累值
            this.unionDonate_costMoney = 0;
            // 357	来自于联盟捐献的消耗	每天总量	积累值
            this.worldShop_costMoney = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_MoneyCost._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_MoneyCost();
            o.key = bb.readInt();
            o.upBuild_costMoney = bb.readInt();
            o.bodyStudy_costMoney = bb.readInt();
            o.trade_costMoney = bb.readInt();
            o.book_costMoney = bb.readInt();
            o.unionDonate_costMoney = bb.readInt();
            o.worldShop_costMoney = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.upBuild_costMoney);
            bb.writeInt(this.bodyStudy_costMoney);
            bb.writeInt(this.trade_costMoney);
            bb.writeInt(this.book_costMoney);
            bb.writeInt(this.unionDonate_costMoney);
            bb.writeInt(this.worldShop_costMoney);
            return bb;
        }
    }
    exports.GI_MoneyCost = GI_MoneyCost;
    GI_MoneyCost._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_MoneyCost", 2257238264, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("upBuild_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("bodyStudy_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("trade_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("book_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("unionDonate_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("worldShop_costMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_MoneyCost._$info.name_hash, GI_MoneyCost, GI_MoneyCost._$info.name);
    //资源变化表(铜币增加)(改变表)//["cityTask","stall","rest","treasure_map","trade","treasureBox","gifts","jjc"]
    class GI_MoneyAdd extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.cityTask_addMoney = 0;
            // 346	来自于日常任务的铜币获得
            this.stall_addMoney = 0;
            // 345	来自于商铺的铜币获得
            this.rest_addMoney = 0;
            // 347	来自于挂机的铜币获得
            this.treasure_map_addMoney = 0;
            // 348	来自于挖宝的铜币获得
            this.trade_addMoney = 0;
            // 352	来自于交易所的获得
            this.treasureBox_addMoney = 0;
            // 349	来自于开袋子的铜币获得
            this.gifts_addMoney = 0;
            // 350	来自于礼包的铜币获得
            this.jjc_addMoney = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_MoneyAdd._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_MoneyAdd();
            o.key = bb.readInt();
            o.cityTask_addMoney = bb.readInt();
            o.stall_addMoney = bb.readInt();
            o.rest_addMoney = bb.readInt();
            o.treasure_map_addMoney = bb.readInt();
            o.trade_addMoney = bb.readInt();
            o.treasureBox_addMoney = bb.readInt();
            o.gifts_addMoney = bb.readInt();
            o.jjc_addMoney = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.cityTask_addMoney);
            bb.writeInt(this.stall_addMoney);
            bb.writeInt(this.rest_addMoney);
            bb.writeInt(this.treasure_map_addMoney);
            bb.writeInt(this.trade_addMoney);
            bb.writeInt(this.treasureBox_addMoney);
            bb.writeInt(this.gifts_addMoney);
            bb.writeInt(this.jjc_addMoney);
            return bb;
        }
    }
    exports.GI_MoneyAdd = GI_MoneyAdd;
    GI_MoneyAdd._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_MoneyAdd", 4108578165, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("cityTask_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("stall_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("rest_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("treasure_map_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("trade_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("treasureBox_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("gifts_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_addMoney", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_MoneyAdd._$info.name_hash, GI_MoneyAdd, GI_MoneyAdd._$info.name);
    //资源变化表(银元宝消耗)(改变表)//["speed","shop"] //[4001,4002,4003,4004,4005,4006,4007,4008,4009,4010,4011]
    class GI_DiamondCost extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.speed_costDiamond = 0;
            // 325	来自加速的直接银元宝消费
            this.shop_costDiamond = 0;
            // 326	来自商城的总银元宝消费
            this.shop_4001_costDiamond = 0;
            // 	铜令牌
            this.shop_4002_costDiamond = 0;
            // 	极寒冰石
            this.shop_4003_costDiamond = 0;
            // 	上古冰石
            this.shop_4004_costDiamond = 0;
            // 	青铜钥匙
            this.shop_4005_costDiamond = 0;
            // 	5分钟加速卡	
            this.shop_4006_costDiamond = 0;
            // 	30分钟加速卡
            this.shop_4007_costDiamond = 0;
            // 	2小时加速卡	
            this.shop_4008_costDiamond = 0;
            // 	10小时加速卡
            this.shop_4009_costDiamond = 0;
            // 黍米黄酒（大曲）
            this.shop_4010_costDiamond = 0;
            // 麦曲黄酒（大曲）
            this.shop_4011_costDiamond = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_DiamondCost._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_DiamondCost();
            o.key = bb.readInt();
            o.speed_costDiamond = bb.readInt();
            o.shop_costDiamond = bb.readInt();
            o.shop_4001_costDiamond = bb.readInt();
            o.shop_4002_costDiamond = bb.readInt();
            o.shop_4003_costDiamond = bb.readInt();
            o.shop_4004_costDiamond = bb.readInt();
            o.shop_4005_costDiamond = bb.readInt();
            o.shop_4006_costDiamond = bb.readInt();
            o.shop_4007_costDiamond = bb.readInt();
            o.shop_4008_costDiamond = bb.readInt();
            o.shop_4009_costDiamond = bb.readInt();
            o.shop_4010_costDiamond = bb.readInt();
            o.shop_4011_costDiamond = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.speed_costDiamond);
            bb.writeInt(this.shop_costDiamond);
            bb.writeInt(this.shop_4001_costDiamond);
            bb.writeInt(this.shop_4002_costDiamond);
            bb.writeInt(this.shop_4003_costDiamond);
            bb.writeInt(this.shop_4004_costDiamond);
            bb.writeInt(this.shop_4005_costDiamond);
            bb.writeInt(this.shop_4006_costDiamond);
            bb.writeInt(this.shop_4007_costDiamond);
            bb.writeInt(this.shop_4008_costDiamond);
            bb.writeInt(this.shop_4009_costDiamond);
            bb.writeInt(this.shop_4010_costDiamond);
            bb.writeInt(this.shop_4011_costDiamond);
            return bb;
        }
    }
    exports.GI_DiamondCost = GI_DiamondCost;
    GI_DiamondCost._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_DiamondCost", 1039171671, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("speed_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4001_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4002_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4003_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4004_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4005_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4006_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4007_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4008_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4009_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4010_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_4011_costDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_DiamondCost._$info.name_hash, GI_DiamondCost, GI_DiamondCost._$info.name);
    //资源变化表(银元宝增加)(改变表)//["season","fund","first","drama","jjc","dayTask"] 
    class GI_DiamondAdd extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.season_addDiamond = 0;
            // 318	总赛季获得银元宝	每天总量	积累值
            this.fund_addDiamond = 0;
            // 319	总基金获得银元宝	每天总量	积累值
            this.first_addDiamond = 0;
            // 320	总首充获得银元宝	每天总量	积累值
            this.drama_addDiamond = 0;
            // 321	总主线获得银元宝	每天总量	积累值
            this.jjc_addDiamond = 0;
            // 322	竞技场领取银元宝数	每天总量	积累值
            this.dayTask_addDiamond = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_DiamondAdd._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_DiamondAdd();
            o.key = bb.readInt();
            o.season_addDiamond = bb.readInt();
            o.fund_addDiamond = bb.readInt();
            o.first_addDiamond = bb.readInt();
            o.drama_addDiamond = bb.readInt();
            o.jjc_addDiamond = bb.readInt();
            o.dayTask_addDiamond = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.season_addDiamond);
            bb.writeInt(this.fund_addDiamond);
            bb.writeInt(this.first_addDiamond);
            bb.writeInt(this.drama_addDiamond);
            bb.writeInt(this.jjc_addDiamond);
            bb.writeInt(this.dayTask_addDiamond);
            return bb;
        }
    }
    exports.GI_DiamondAdd = GI_DiamondAdd;
    GI_DiamondAdd._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_DiamondAdd", 2793805257, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("season_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("fund_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("first_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("drama_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("jjc_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("dayTask_addDiamond", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_DiamondAdd._$info.name_hash, GI_DiamondAdd, GI_DiamondAdd._$info.name);
    //rmb充值统计表//["fund","diamond","gifts"]
    class GI_RechargeRMB extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.total_RMB = 0;
            // 295	总充值RMB
            this.fund_RMB = 0;
            // 296	来自基金的充值RMB
            this.diamond_RMB = 0;
            // 297	来自充值界面的充值RMB
            this.gifts_RMB = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_RechargeRMB._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_RechargeRMB();
            o.key = bb.readInt();
            o.total_RMB = bb.readInt();
            o.fund_RMB = bb.readInt();
            o.diamond_RMB = bb.readInt();
            o.gifts_RMB = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.total_RMB);
            bb.writeInt(this.fund_RMB);
            bb.writeInt(this.diamond_RMB);
            bb.writeInt(this.gifts_RMB);
            return bb;
        }
    }
    exports.GI_RechargeRMB = GI_RechargeRMB;
    GI_RechargeRMB._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_RechargeRMB", 609180768, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("total_RMB", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("fund_RMB", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("diamond_RMB", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("gifts_RMB", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_RechargeRMB._$info.name_hash, GI_RechargeRMB, GI_RechargeRMB._$info.name);
    //资源变化表(金元宝消耗)(改变表) : 统计金元宝的产出//["gifts","diamond_recharge","fund","trade"]
    class GI_JinAdd extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.recharge_addJin = 0;
            // 299	总充值获得金元宝        (礼包+商城充值+基金等充值获得的金元宝)
            this.gifts_addJin = 0;
            // 300	总赠送获得金元宝        (礼包等也会赠送)
            this.diamond_recharge_addJin = 0;
            // 301	来自商城充值的金元宝    (商城元宝充值)
            this.fund_addJin = 0;
            // 302	来自基金的领取金元宝    (基金)
            this.trade_addJin = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_JinAdd._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_JinAdd();
            o.key = bb.readInt();
            o.recharge_addJin = bb.readInt();
            o.gifts_addJin = bb.readInt();
            o.diamond_recharge_addJin = bb.readInt();
            o.fund_addJin = bb.readInt();
            o.trade_addJin = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.recharge_addJin);
            bb.writeInt(this.gifts_addJin);
            bb.writeInt(this.diamond_recharge_addJin);
            bb.writeInt(this.fund_addJin);
            bb.writeInt(this.trade_addJin);
            return bb;
        }
    }
    exports.GI_JinAdd = GI_JinAdd;
    GI_JinAdd._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_JinAdd", 3147504179, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("recharge_addJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("gifts_addJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("diamond_recharge_addJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("fund_addJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("trade_addJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_JinAdd._$info.name_hash, GI_JinAdd, GI_JinAdd._$info.name);
    //资源变化表(金元宝消耗)(改变表) : 统计金元宝在各功能的消耗//["trade","tili","speed","season","season_level","shop"]//[1001,1002,1003,1004,1005,1006,1007,1008,1009,1010,1011,1012,1013,1014,1015]
    class GI_JinCost extends struct_mgr_1.Struct {
        constructor() {
            super(...arguments);
            //id
            this.trade_costJin = 0;
            // 305	交易所总失去金元宝
            this.tili_costJin = 0;
            // 305	购买体力花费金元宝
            this.speed_costJin = 0;
            // 307    来自加速的金元宝消费
            this.season_costJin = 0;
            // 315	来自赛季的邀请函的购买消费
            this.season_level_costJin = 0;
            // 316	来自赛季购买等级的购买消费
            this.shop_costJin = 0;
            // 306	来自商城的道具总金元宝消费
            this.shop_1001_costJin = 0;
            // 来自商城  亮银钥匙
            this.shop_1002_costJin = 0;
            // 来自商城  黄金钥匙
            this.shop_1003_costJin = 0;
            // 来自商城  银喇叭
            this.shop_1004_costJin = 0;
            // 来自商城  金喇叭
            this.shop_1005_costJin = 0;
            // 来自商城  李白剑经
            this.shop_1006_costJin = 0;
            // 来自商城  酒曲
            this.shop_1007_costJin = 0;
            // 来自商城  酒花
            this.shop_1008_costJin = 0;
            // 来自商城  突破石
            this.shop_1009_costJin = 0;
            // 来自商城  5分钟加速卡
            this.shop_1010_costJin = 0;
            // 来自商城  30分钟加速卡
            this.shop_1011_costJin = 0;
            // 来自商城  2小时加速卡
            this.shop_1012_costJin = 0;
            // 来自商城  10小时加速卡
            this.shop_1013_costJin = 0;
            // 来自商城  黍米黄酒（佳酿）
            this.shop_1014_costJin = 0;
            // 来自商城  麦曲黄酒（佳酿）
            this.shop_1015_costJin = 0;
        }
        addMeta(mgr) {
            if (this._$meta) return;
            struct_mgr_1.addToMeta(mgr, this);
        }
        removeMeta() {
            struct_mgr_1.removeFromMeta(this);
        }
        static bonType() {
            return GI_JinCost._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GI_JinCost();
            o.key = bb.readInt();
            o.trade_costJin = bb.readInt();
            o.tili_costJin = bb.readInt();
            o.speed_costJin = bb.readInt();
            o.season_costJin = bb.readInt();
            o.season_level_costJin = bb.readInt();
            o.shop_costJin = bb.readInt();
            o.shop_1001_costJin = bb.readInt();
            o.shop_1002_costJin = bb.readInt();
            o.shop_1003_costJin = bb.readInt();
            o.shop_1004_costJin = bb.readInt();
            o.shop_1005_costJin = bb.readInt();
            o.shop_1006_costJin = bb.readInt();
            o.shop_1007_costJin = bb.readInt();
            o.shop_1008_costJin = bb.readInt();
            o.shop_1009_costJin = bb.readInt();
            o.shop_1010_costJin = bb.readInt();
            o.shop_1011_costJin = bb.readInt();
            o.shop_1012_costJin = bb.readInt();
            o.shop_1013_costJin = bb.readInt();
            o.shop_1014_costJin = bb.readInt();
            o.shop_1015_costJin = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.key);
            bb.writeInt(this.trade_costJin);
            bb.writeInt(this.tili_costJin);
            bb.writeInt(this.speed_costJin);
            bb.writeInt(this.season_costJin);
            bb.writeInt(this.season_level_costJin);
            bb.writeInt(this.shop_costJin);
            bb.writeInt(this.shop_1001_costJin);
            bb.writeInt(this.shop_1002_costJin);
            bb.writeInt(this.shop_1003_costJin);
            bb.writeInt(this.shop_1004_costJin);
            bb.writeInt(this.shop_1005_costJin);
            bb.writeInt(this.shop_1006_costJin);
            bb.writeInt(this.shop_1007_costJin);
            bb.writeInt(this.shop_1008_costJin);
            bb.writeInt(this.shop_1009_costJin);
            bb.writeInt(this.shop_1010_costJin);
            bb.writeInt(this.shop_1011_costJin);
            bb.writeInt(this.shop_1012_costJin);
            bb.writeInt(this.shop_1013_costJin);
            bb.writeInt(this.shop_1014_costJin);
            bb.writeInt(this.shop_1015_costJin);
            return bb;
        }
    }
    exports.GI_JinCost = GI_JinCost;
    GI_JinCost._$info = new sinfo_1.StructInfo("app_c/demo/pressure_test/server/gi_res.GI_JinCost", 3991065999, new Map([["primary", "key"], ["db", "memory"], ["dbMonitor", "true"]]), [new sinfo_1.FieldInfo("key", new sinfo_1.EnumType(sinfo_1.Type.I32), null), new sinfo_1.FieldInfo("trade_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("tili_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("speed_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("season_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("season_level_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1001_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1002_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1003_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1004_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1005_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1006_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1007_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1008_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1009_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1010_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1011_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1012_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1013_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1014_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]])), new sinfo_1.FieldInfo("shop_1015_costJin", new sinfo_1.EnumType(sinfo_1.Type.Usize), new Map([["default", "0"]]))]);
    struct_mgr_1.structMgr.register(GI_JinCost._$info.name_hash, GI_JinCost, GI_JinCost._$info.name);
});