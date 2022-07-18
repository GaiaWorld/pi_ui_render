_$pi.define("app_c/demo/pressure_test/server/gi_cfg", ["require", "exports", "module", "pi_pt/db/gi_listen", "pi_pt/db/gi_listen_cfg.struct"], function (require, exports, module, gi_listen_1, gi_listen_cfg_struct_1) {
    "use strict";

    exports.initGiCfg = void 0;
    /**
     *   table: string;
     *   values: Array<string>;
     *   is_last: boolean;
     *   appid: string;
     */
    const listen = [{
        table: "app_c/demo/pressure_test/server/gi_prop.GI_NormalPropRecord",
        values: ["add_14001", "add_14002", "add_14004", "add_14005"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_JJCPropRecord",
        values: ["jjc_add_10019", "jjc_add_17001", "jjc_add_21044", "jjc_add_61036"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_UnionShopPropRecord",
        values: ["union_shop_add_15001", "union_shop_add_15002", "union_shop_add_21000", "union_shop_add_21001"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_UnionSeasonPropRecord",
        values: ["union_season_add_10016", "union_season_add_10017", "union_season_add_17001", "union_season_add_17003"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_NewWorldPropRecord",
        values: ["world_add_15001", "world_add_15002", "world_add_15004", "world_add_15006"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_NewWorldFuBenPropRecord",
        values: ["world_fb_add_15001", "world_fb_add_15002", "world_fb_add_15004", "world_fb_add_15007"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_prop.GI_BluePrintFuncRecord",
        values: ["blue_level_55", "blue_level_60", "blue_level_65", "blue_level_80"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_res.GI_ChangeCoinRecord",
        values: ["cost_11006", "cost_11008", "cost_11001", "cost_10011"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_res.GI_MoneyCost",
        values: ["trade_costMoney", "upBuild_costMoney", "worldShop_costMoney", "bodyStudy_costMoney"],
        is_last: false
    }, {
        table: "app_c/demo/pressure_test/server/gi_res.GI_DiamondCost",
        values: ["shop_4001_costDiamond", "shop_4002_costDiamond", "shop_4004_costDiamond", "shop_4007_costDiamond"],
        is_last: false
    }];
    exports.initGiCfg = () => {
        const giCfg = new gi_listen_cfg_struct_1.GiCfg();
        giCfg.app_id = 'pi_demo';
        giCfg.bi_ser = 'http://bitest.highapp.com:8099';
        giCfg.listen = initCfg(listen);
        gi_listen_1.registerCfg(giCfg);
    };
    const initCfg = listen => {
        const r = [];
        listen.forEach(element => {
            const cfg = new gi_listen_cfg_struct_1.Cfg();
            cfg.table = element.table;
            cfg.values = element.values;
            cfg.is_last = element.is_last;
            cfg.appid = element.appid;
            r.push(cfg);
        });
        return r;
    };
});