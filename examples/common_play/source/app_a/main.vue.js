_$pi.define("app_a/main.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./main.vue.tpl", "app/mod/pi", "app/mod/root", "app_a/user/base/user_base", "design/activity/day7_type.struct", "app_b/base/condition_func", "design/activity/box_award.struct", "app_c/day7_activity/client/base/base"], function (require, exports, module, direct_1, main_vue_tpl_1, pi_1, root_1, user_base_1, day7_type_struct_1, condition_func_1, box_award_struct_1, base_1) {
    "use strict";

    exports.initMeta = exports.SmallTypeBtn = void 0;
    const redCondition = {};
    class Day7Widget {
        constructor() {
            this.name = module.name;
            this.bgTypes = [];
            this.smallTypes = [];
            this.is_scroll = false;
            this.redCondition = redCondition;
            this.selectBigTypeId = 1;
            this.selectSmallTypeId = 1;
            this.coinCfg = null;
            //货币数量
            this.value = 0;
        }
        //获取货币数量

        getCoinValue() {
            try {
                const _this = this;

                const _temp = function () {
                    if (_this.coinCfg) {
                        return Promise.resolve(condition_func_1.funModGetShowValue(_this.coinCfg.condition, _this.coinCfg.param)).then(function (_condition_func_1$fun) {
                            _this.value = _condition_func_1$fun;
                        });
                    }
                }();

                return Promise.resolve(_temp && _temp.then ? _temp.then(function () {}) : void 0);
            } catch (e) {
                return Promise.reject(e);
            }
        }

        create() {
            try {
                const _this2 = this;

                _this2.bgTypes = getBigTypes();
                _this2.selectBigTypeId = _this2.bgTypes[0].id;
                _this2.smallTypes = user_base_1.getCfg(day7_type_struct_1.Day7Type, _this2.selectBigTypeId).smallTypes;
                _this2.selectSmallTypeId = _this2.smallTypes[0];
                _this2.is_scroll = _this2.bgTypes.length >= 5;
                const cfg = getBoxCoinId();

                const _temp2 = function () {
                    if (cfg) {
                        _this2.coinCfg = cfg;
                        return Promise.resolve(_this2.getCoinValue()).then(function () {});
                    }
                }();

                return Promise.resolve(_temp2 && _temp2.then ? _temp2.then(function () {}) : void 0);
            } catch (e) {
                return Promise.reject(e);
            }
        }
        //更新箱子货币数量


        updateValue() {
            try {
                const _this3 = this;

                return Promise.resolve(_this3.getCoinValue()).then(function () {});
            } catch (e) {
                return Promise.reject(e);
            }
        }
        //关闭界面


        handleGoBack() {
            root_1.close(this);
        }
        //切换大类型
        changeBigType(index) {
            const id = this.bgTypes[index].id;
            if (this.selectBigTypeId !== id) {
                this.selectBigTypeId = id;
            }
        }
        //切换小类型
        handleChangeSmallType(id) {
            if (this.selectSmallTypeId !== id) {
                this.selectSmallTypeId = id;
            }
        }
    }
    exports.default = Day7Widget;
    Day7Widget.group = "secondary";
    Day7Widget.path = module.name;
    //小类型按钮
    class SmallTypeBtn {
        constructor() {
            this.id = 1;
            this.name = "";
            this.bg = "dwbtn_btn2";
            this.textClass = "hwbtn_hb_txt2";
        }
        create() {
            this.name = user_base_1.getCfg(day7_type_struct_1.Day7TypeName, this.id).name;
        }
        watchSelect() {
            this.bg = this.select ? "dwbtn_btn" : "dwbtn_btn2";
            this.textClass = this.select ? "hwbtn_hb_txt" : "hwbtn_hb_txt2";
        }
    }
    exports.SmallTypeBtn = SmallTypeBtn;
    //获取大类型
    const getBigTypes = () => {
        const cfg = user_base_1.getCfg(day7_type_struct_1.Day7Type);
        const list = [];
        for (let v of cfg) {
            if (condition_func_1.funMod(v[1].condition, v[1].param)) {
                list.push({
                    id: v[0],
                    name: v[1].name,
                    small: v[1].smallTypes
                });
            }
        }
        return list;
    };
    //获取箱子关心的货币配置
    const getBoxCoinId = () => {
        const cfg = user_base_1.getCfg(box_award_struct_1.BoxAward);
        for (let v of cfg) {
            if (v[1].act_id === base_1.Day7Id) {
                return { condition: v[1].condition, param: v[1].param };
            }
        }
    };
    pi_1.globalReceive("dya7", () => {
        root_1.open(Day7Widget);
    });
    exports.initMeta = () => {
        let _$tpl = "app_a/main.vue.tpl.ts",
            _$cssPath = "app_a/main.vue.wcss",
            _$cssHash = 272981311;
        SmallTypeBtn["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: main_vue_tpl_1.BW20, cssHash: _$cssHash };
        Day7Widget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: main_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addWatch(SmallTypeBtn, "watchSelect", function (info) {
        return info.dirty0 & 8;
    });
    direct_1.defineAccessors(Day7Widget, ["value", "selectBigTypeId", "selectSmallTypeId", "smallTypes", "is_scroll", "bgTypes", "redCondition"]);
    direct_1.defineAccessors(SmallTypeBtn, ["bg", "name", "textClass"]);
    direct_1.defineAccessors(SmallTypeBtn, ['select']);
    direct_1.addField(SmallTypeBtn, ['id']);
    direct_1.addField(Day7Widget, ['name']);
});