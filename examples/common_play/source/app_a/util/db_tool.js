_$pi.define("app_a/util/db_tool", ["require", "exports", "module", "pi_utils/net/rpc_r.struct", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo", "../struct/db_role.struct", "pi_common/constant", "pi_utils/math/bigint/biginteger"], function (require, exports, module, rpc_r_struct_1, struct_mgr_1, sinfo_1, db_role_struct_1, constant_1, bigInt) {
    "use strict";

    exports.addGold = exports.getGold = exports.initialTab = exports.queryMore = exports.getAllBaseCfgByTr = exports.queryBaseCfgByTr = exports.queryBaseByDb = exports.getBaseByDb = exports.initBaseResult = void 0;
    // import { getLogger } from "pi_utils/util/logger";
    // declare var module;
    // const {logV, logD, logI, logW, logE} = getLogger(module.name, "app");
    /**
     * 初始化基础返回
     */
    exports.initBaseResult = result => {
        const r = new rpc_r_struct_1.OK_I();
        r.value = result;
        return r;
    };
    /**
     * 获取通用数据库表
     */
    exports.getBaseByDb = function (tr, id, tabName, descending) {
        try {
            return Promise.resolve(tr.iter_raw(constant_1.WARE_NAME, tabName, id, descending, null)).then(function (iterBase) {
                // 取from表的迭代器
                const elBase = iterBase.next();
                return elBase ? elBase[1] : null;
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    /**
     * 获取通用数据库表
     */
    exports.queryBaseByDb = function (tr, id, tabName, wareType) {
        try {
            return Promise.resolve(tr.query([{ ware: wareType || constant_1.WARE_NAME, tab: tabName, key: id }], 1000, false)).then(function (_tr$query) {
                const elBase = _tr$query[0];
                return elBase.value ? elBase.value : parseTable(tabName, id);
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    /**
     * 获取通用配置表
     */
    exports.queryBaseCfgByTr = function (tr, id, tabName) {
        try {
            return Promise.resolve(tr.query([{ ware: constant_1.MEMORY_NAME, tab: tabName, key: id }], 1000, false)).then(function (_tr$query2) {
                const elBase = _tr$query2[0];
                return elBase.value ? elBase.value : null;
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    /**
     * 获取通用配置表所有数据
     */
    exports.getAllBaseCfgByTr = function (tr, tabName, isMap) {
        try {
            const cfgs = isMap ? new Map() : [];
            return Promise.resolve(tr.iter_raw(constant_1.MEMORY_NAME, tabName, null, false, null)).then(function (iterCfg) {
                // 取from表的迭代器
                let el = iterCfg.next();
                while (el) {
                    if (isMap) {
                        cfgs.set(el[0], el[1]);
                    } else {
                        cfgs.push(el[1]);
                    }
                    el = iterCfg.next();
                }
                return cfgs;
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    // 查询更多的
    exports.queryMore = function (tr, arr) {
        try {
            return Promise.resolve(tr.query(arr, 1000, false)).then(function (_tr$query3) {
                return _tr$query3.map(v => {
                    if (!v.value) v.value = parseTable(v.tab, v.key);
                    return v;
                });
            });
        } catch (e) {
            return Promise.reject(e);
        }
    };
    const TypeDefaultValue = {
        [sinfo_1.Type.Bool]: () => false,
        [sinfo_1.Type.U8]: () => 0,
        [sinfo_1.Type.U16]: () => 0,
        [sinfo_1.Type.U32]: () => 0,
        [sinfo_1.Type.U64]: () => 0,
        [sinfo_1.Type.U128]: () => 0,
        [sinfo_1.Type.U256]: () => 0,
        [sinfo_1.Type.Usize]: () => 0,
        [sinfo_1.Type.I8]: () => 0,
        [sinfo_1.Type.I16]: () => 0,
        [sinfo_1.Type.I32]: () => 0,
        [sinfo_1.Type.I64]: () => 0,
        [sinfo_1.Type.I128]: () => 0,
        [sinfo_1.Type.I256]: () => 0,
        [sinfo_1.Type.Isize]: () => 0,
        [sinfo_1.Type.F32]: () => 0,
        [sinfo_1.Type.F64]: () => 0,
        [sinfo_1.Type.BigI]: () => 0,
        [sinfo_1.Type.Str]: () => '',
        [sinfo_1.Type.Arr]: () => [],
        [sinfo_1.Type.Map]: () => new Map(),
        [sinfo_1.Type.Struct]: (Tab, v) => initialTab(Tab, v)
    };
    /**
     * 默认约定，一个结构体第一个字段为主键，如果不是主键则 idOrSpecialKey 为{ 'xxxkey' :'123'} 形式传入主键
     * 结构体中包含另外一个子结构体则可以 {'anothorStruct' :{k1:'v1',k2:'v2'}}
     * 生成对象并赋默认值, 根据需要可自行调整
     * @param Struct 结构体
     * @param idOrSpecialKey 初始化数值
     */
    function initialTab(_Struct, idOrSpecialKey) {
        let fields = _Struct._$info.fields;
        let obj = new _Struct();
        for (let i = 0; i < fields.length; i++) {
            let field = fields[i],
                key = field.name;
            // 默认设置主键
            if (i == 0 && idOrSpecialKey && typeof idOrSpecialKey != 'object') {
                obj[key] = idOrSpecialKey;
                continue;
            }
            if (field.ftype.type == sinfo_1.Type.Struct) {
                let structInfo = field.ftype.structType;
                let struct = struct_mgr_1.structMgr.getConstructor(structInfo.name_hash);
                obj[key] = TypeDefaultValue[sinfo_1.Type.Struct](struct, idOrSpecialKey[key]);
                continue;
            }
            obj[key] = idOrSpecialKey && idOrSpecialKey[key] || TypeDefaultValue[field.ftype.type]();
        }
        return obj;
    }
    exports.initialTab = initialTab;
    const parseTable = (tableName, key) => {
        let r = null;
        switch (tableName) {
            case db_role_struct_1.CurrencyDb._$info.name:
                r = initialTab(db_role_struct_1.CurrencyDb, key);
                break;
            default:
        }
        return r;
    };
    /**
     * 获取当前金币1
     */
    exports.getGold = money => {
        const curr = bigInt(money.gold.add, 10).subtract(bigInt(money.gold.cost, 10)).valueOf();
        if (curr === Number.POSITIVE_INFINITY) {
            return 1e300;
        } else if (curr === Number.NEGATIVE_INFINITY) {
            return 0;
        } else {
            return curr;
        }
    };
    /**
     * 添加当前金币1
     */
    exports.addGold = (money, value) => {
        if (value < 0) return;
        money.gold.add = bigInt(Math.round(value)).add(bigInt(money.gold.add, 10)).toString();
    };
});