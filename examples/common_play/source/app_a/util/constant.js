_$pi.define("app_a/util/constant", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";

    exports.parseRes = exports.ERROR_MSG = exports.ERROR_CODE = exports.MAX_MSG_COUNT = exports.MOBILE_MSG_ID = exports.WORLD_GROUP_ID = exports.MAX_ROLE_ID = exports.BOOT_EVENT = exports.r_ok = exports.ARENA_RANK_CHANNEL = exports.OFFLINE_RPC_LIST = exports.DB_VERSION = exports.CDB_VERSION = exports.SERVER_ID = exports.ROLE_ACCOUNT = exports.USER_ID = void 0;
    // 用户及角色主键
    exports.USER_ID = 'uid';
    exports.ROLE_ACCOUNT = "account";
    exports.SERVER_ID = 'serverId';
    // 前端数据库版本哈希表
    exports.CDB_VERSION = 'cdb_version';
    // 后端数据库版本哈希表
    exports.DB_VERSION = 'db_version';
    // 离线状态代理服务器RPC操作指令
    exports.OFFLINE_RPC_LIST = 'offline_rpc_list';
    exports.ARENA_RANK_CHANNEL = 9;
    exports.r_ok = 1;
    var BOOT_EVENT;
    (function (BOOT_EVENT) {
        BOOT_EVENT["APP_B"] = "app_b:boot";
        BOOT_EVENT["APP_B_NEXT"] = "app_b:next";
    })(BOOT_EVENT = exports.BOOT_EVENT || (exports.BOOT_EVENT = {}));
    exports.MAX_ROLE_ID = 'MAX_ROLE_ID';
    exports.WORLD_GROUP_ID = 'WORLD_GROUP_ID';
    exports.MOBILE_MSG_ID = 'MOBILE_MSG_ID';
    // 最大获取消息数
    exports.MAX_MSG_COUNT = 20;
    // 错误码
    var ERROR_CODE;
    (function (ERROR_CODE) {
        ERROR_CODE[ERROR_CODE["SignError"] = -1] = "SignError";
        ERROR_CODE[ERROR_CODE["ParamError"] = -2] = "ParamError";
        ERROR_CODE[ERROR_CODE["RoleNotExist"] = -3] = "RoleNotExist";
        ERROR_CODE[ERROR_CODE["NotAllowed"] = -4] = "NotAllowed";
        ERROR_CODE[ERROR_CODE["AlreadyDone"] = -5] = "AlreadyDone";
        ERROR_CODE[ERROR_CODE["NotMatch"] = -6] = "NotMatch";
        ERROR_CODE[ERROR_CODE["NoLogin"] = -7] = "NoLogin";
        ERROR_CODE[ERROR_CODE["NotFriend"] = -8] = "NotFriend";
        ERROR_CODE[ERROR_CODE["DBError"] = -9] = "DBError";
        ERROR_CODE[ERROR_CODE["LevelNotEnough"] = -10] = "LevelNotEnough";
    })(ERROR_CODE = exports.ERROR_CODE || (exports.ERROR_CODE = {}));
    exports.ERROR_MSG = {
        [ERROR_CODE.SignError]: '签名错误',
        [ERROR_CODE.ParamError]: '参数错误',
        [ERROR_CODE.RoleNotExist]: '角色不存在',
        [ERROR_CODE.NotAllowed]: '不被允许',
        [ERROR_CODE.AlreadyDone]: '已经做过',
        [ERROR_CODE.NotMatch]: '不符合条件',
        [ERROR_CODE.NoLogin]: '未登录',
        [ERROR_CODE.NotFriend]: '不是好友',
        [ERROR_CODE.DBError]: '数据库错误',
        [ERROR_CODE.LevelNotEnough]: '等级不足'
    };
    // 解析返回值
    exports.parseRes = r => {
        if (r === exports.r_ok) return 'success';
        return typeof r === 'number' ? exports.ERROR_MSG[r] : r;
    };
});