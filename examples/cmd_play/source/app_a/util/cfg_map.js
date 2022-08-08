_$pi.define("app_a/util/cfg_map", ["require", "exports", "module", "pi_utils/util/cfg"], function (require, exports, module, cfg_1) {
    "use strict";

    exports.tableNameAddKey = exports.getMap = void 0;
    /**
     * 处理配置表获取
     */
    // 获取map
    exports.getMap = (table, key) => {
        if (typeof table !== 'string') {
            table = exports.tableNameAddKey(table._$info.name, table._$info.notes.get('primary'));
        }
        if (!cfg_1.cfgMgr.map.has(table)) return;
        if (key !== undefined && key !== null) return cfg_1.cfgMgr.map.get(table).get(key);
        return cfg_1.cfgMgr.map.get(table);
    };
    /**
     * 表名与主键的组合
     */
    exports.tableNameAddKey = (tableName, key) => {
        if (!tableName || !key) return tableName;
        return `${tableName}#${key}`;
    };
});