_$pi.define("app_a/login/server/base", ["require", "exports", "module", "app_a/struct/db_role.struct", "pi_common/constant", "pi_utils/net/autologin_new"], function (require, exports, module, db_role_struct_1, constant_1, autologin_new_1) {
    "use strict";

    exports.initRole = exports.getCurrentRid = void 0;
    // 获取当前用户rid
    exports.getCurrentRid = () => {
        return Number(autologin_new_1.getSession(constant_1.ROLE_ID));
    };
    // 初始化角色信息
    exports.initRole = rid => {
        const dbRole = new db_role_struct_1.RoleDb();
        dbRole.rid = rid;
        dbRole.name = '游客' + rid;
        dbRole.plat = 'sso';
        dbRole.serverId = 1;
        dbRole.styleId = 1;
        dbRole.startTime = Date.now();
        return dbRole;
    };
});