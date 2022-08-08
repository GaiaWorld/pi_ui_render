_$pi.define("app_c/util/tools", ["require", "exports", "module", "app_a/util/constant", "app_a/util/new_store", "app_a/util/setup", "app_c/chat/server/chat.struct", "app_c/chat/server/chat.topic", "pi_common/im/server/db/message.struct"], function (require, exports, module, constant_1, new_store_1, setup_1, chat_struct_1, chat_topic_1, message_struct_1) {
    "use strict";

    exports.getMsgList = exports.getUserHistoryMsg = exports.dealWorldChatList = exports.dealUserChatList = exports.firstGetUserCursor = void 0;
    // 首次进入 获取聊天游标
    exports.firstGetUserCursor = () => {
        setup_1.clientRpc(chat_topic_1.getUserCursor).then(r => {
            console.log('getUserCursor', r);
            const map = new_store_1.newStore.find('userCursor');
            for (const v of r) {
                map.set(v.rid, v);
            }
            new_store_1.newStore.notify('userCursor', map);
            // TODO 按需获取每个好友的离线消息 即游标与最新ID不等
        });
    };
    // 处理好友聊天记录
    exports.dealUserChatList = r => {
        return new Promise(resolve => {
            const newId = r.key[r.key.length - 1];
            const rrid = r.key.find(v => v !== new_store_1.newStore.find('rid')); // 对方rid
            let list = new_store_1.newStore.find('friendChatList').get(rrid);
            if (!list) {
                list = [];
                new_store_1.newStore.find('friendChatList').set(rrid, list);
            }
            let cursor = new_store_1.newStore.find('userCursor').get(rrid);
            if (!cursor) {
                cursor = new chat_struct_1.UserCursor();
                cursor.rid = rrid;
                cursor.cursor = 0;
                cursor.newId = newId;
                new_store_1.newStore.find('userCursor').set(rrid, cursor);
            }
            const lastId = cursor.cursor;
            let count = newId - lastId;
            // 最多获取20条
            count = count < constant_1.MAX_MSG_COUNT ? count : constant_1.MAX_MSG_COUNT;
            // 主动获取消息记录 更新后端游标
            exports.getMsgList(message_struct_1.CONV_TYPE.CONV_C2C, rrid, r.key, count).then(res => {
                updateMsgList(list, res);
                // 更新本地游标
                cursor.cursor = newId;
                resolve(list);
            });
        });
    };
    // 处理世界聊天记录
    exports.dealWorldChatList = r => {
        return new Promise(resolve => {
            const list = new_store_1.newStore.find('groupChatList');
            const last = list[list.length - 1];
            // 没有前一条消息 
            if (!last) {
                list.push(r);
                return resolve(list);
            }
            let count = r.key[r.key.length - 1] - last.key[last.key.length - 1];
            // ID相邻，顺序正确
            if (count === 1) {
                list.push(r);
                return resolve(list);
            }
            // ID不相邻，顺序错误，主动获取消息记录
            exports.getMsgList(message_struct_1.CONV_TYPE.CONV_GROUP, r.key[0], r.key, count).then(res => {
                updateMsgList(list, res);
                resolve(list);
            });
        });
    };
    exports.getUserHistoryMsg = friend => {
        let list = new_store_1.newStore.find('friendChatList').get(friend);
        if (!list) {
            list = [];
            new_store_1.newStore.find('friendChatList').set(friend, list);
        }
        const lastKey = list.length > 0 ? list[list.length - 1].key : [];
        return exports.getMsgList(message_struct_1.CONV_TYPE.CONV_C2C, friend, lastKey, constant_1.MAX_MSG_COUNT).then(res => {
            updateMsgList(list, res);
            return list;
        });
    };
    // 获取消息记录
    exports.getMsgList = (conv_type, from, last_key, count) => {
        const arg = new chat_struct_1.GetMsgParam();
        arg.from = from;
        arg.conv_type = conv_type;
        arg.count = count;
        arg.last_key = last_key;
        arg.msg_class = message_struct_1.MSG_CLASS.MSG;
        return setup_1.clientRpc(chat_topic_1.getMessageList, arg).then(res => {
            console.log('getMessageList', res);
            return res;
        });
    };
    // 更新历史消息
    const updateMsgList = (list, newList) => {
        if (newList.length === 0) return;
        if (list.length === 0) {
            list.push(...newList);
            return;
        }
        const firstId = list[0].key.slice(-1)[0];
        const lastId = list[list.length - 1].key.slice(-1)[0];
        // 过滤掉重复消息
        newList = newList.filter(v => {
            const convId = v.key.slice(-1)[0];
            return lastId !== convId && firstId !== convId;
        });
        if (newList.length === 0) return;
        const newId = newList[0].key.slice(-1)[0];
        // 新的第一条消息大于本地最后一条，表示往后加载，否则往前加载历史记录
        if (newId > lastId) {
            list.push(...newList);
        } else {
            list.unshift(...newList);
        }
        console.log('updateMsgList list', list);
    };
});