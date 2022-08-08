_$pi.define("app_a/util/new_store", ["require", "exports", "module", "pi_utils/util/event"], function (require, exports, module, event_1) {
    "use strict";

    exports.newStore = void 0;
    var newStore;
    (function (newStore) {
        const handlerMap = new event_1.HandlerMap();
        let store = {
            gid: 0,
            mid: 0,
            friendChat: null,
            groupChat: null,
            mobileMsg: null,
            sysMessage: [],
            friendChatList: new Map(),
            groupChatList: [],
            userCursor: new Map()
        };
        // 获取数据
        newStore.find = key => store[key];
        /**
         * 更新store
         */
        newStore.notify = (key, data) => {
            store[key] = data;
            handlerMap.notify(key, data);
        };
        /**
         * 消息处理器
         */
        newStore.register = (key, cb) => {
            handlerMap.add(key, cb);
        };
        newStore.unregister = (key, cb) => {
            handlerMap.remove(key, cb);
        };
    })(newStore = exports.newStore || (exports.newStore = {}));
});