_$pi.define("app_a/util/db_init", ["require", "exports", "module", "pi_pt/db/dblistener", "pi_utils/util/util", "pi_utils/serialization/bon", "pi_utils/serialization/struct_mgr", "pi_common/constant", "pi_pt/native/pi_serv_lib/js_net", "app_a/struct/db_role.struct", "pi_utils/util/logger"], function (require, exports, module, dblistener_1, util_1, bon_1, struct_mgr_1, constant_1, js_net_1, db_role_struct_1, logger_1) {
    "use strict";

    exports.initDBMonitor = exports.setDBMonitor = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    // 绑定用户数据库监听
    exports.setDBMonitor = uid => {
        const key = util_1.ab2hex(new bon_1.BonBuffer().write(uid).getBuffer());
        //ex:
        setTopic(`${constant_1.WARE_NAME_LOG_FILE}.${db_role_struct_1.CurrencyDb._$info.name}.${key}`);
        setTopic(`${constant_1.WARE_NAME_LOG_FILE}.${db_role_struct_1.RoleDb._$info.name}.${key}`);
    };
    // 设置topic
    const setTopic = key => {
        js_net_1.add_global_mqtt_topic("*", false, key);
    };
    // // 创建一个监听名单
    // export const initDBMonitor = () => {
    //     const listener = new DBMqttListener("mqttServer");
    //     const set = new Set<string>();
    //     const handleFn = (c) => {
    //         if (c._$info) {
    //             if (c._$info.notes && c._$info.notes.get('dbMonitor')) {
    //                 set.add(c._$info.name);
    //             }
    //         }
    //     }
    //     structMgr.numberMap.forEach((meta) => {
    //         handleFn(meta.info);
    //     })
    //     listener.setWihteList(set);
    //     let dbMgr: Mgr = env.get("dbMgr");
    //     dbMgr.addListener("sendMqtt", new DBMqttListener("mqttServer"))
    // };
    // 绑定数据库监听
    exports.initDBMonitor = () => {
        // logD('-------addListener start---------');
        const listener = new dblistener_1.DBMqttListener("mqttServer");
        const set = new Set();
        const handleFn = c => {
            // logD('-------addListener handleFn---------',c.notes,c.name);
            if (c.notes && c.notes.get('dbMonitor')) {
                set.add(c.notes.get("db") + "." + c.name);
            }
        };
        struct_mgr_1.structMgr.numberMap.forEach(meta => handleFn(meta.info));
        // logD('-------addListener start---------',structMgr.numberMap);
        listener.setWihteList(set);
        env.dbMgr.addListener("sendMqtt", new dblistener_1.DBMqttListener("mqttServer"));
        // logD('-------addListener over---------');
    };
});