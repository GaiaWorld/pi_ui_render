_$pi.define("app_c/chat/server/chat.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.RpcResult = exports.UserCursor = exports.GetMsgParam = exports.MsgReadParam = exports.SendMsgParam = void 0;
    class SendMsgParam extends struct_mgr_1.Struct {
        static bonType() {
            return SendMsgParam._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new SendMsgParam();
            o.conv_type = bb.readInt();
            o.to = bb.readInt();
            o.mtype = bb.readInt();
            o.msg = bb.readUtf8();
            o.msg_class = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.conv_type);
            bb.writeInt(this.to);
            bb.writeInt(this.mtype);
            bb.writeUtf8(this.msg);
            bb.writeInt(this.msg_class);
            return bb;
        }
    }
    exports.SendMsgParam = SendMsgParam;
    SendMsgParam._$info = new sinfo_1.StructInfo("app_c/chat/server/chat.SendMsgParam", 526482533, null, [new sinfo_1.FieldInfo("conv_type", new sinfo_1.EnumType(sinfo_1.Type.U8), null), new sinfo_1.FieldInfo("to", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("mtype", new sinfo_1.EnumType(sinfo_1.Type.U8), null), new sinfo_1.FieldInfo("msg", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("msg_class", new sinfo_1.EnumType(sinfo_1.Type.U8), null)]);
    struct_mgr_1.structMgr.register(SendMsgParam._$info.name_hash, SendMsgParam, SendMsgParam._$info.name);
    // 消息已读
    class MsgReadParam extends struct_mgr_1.Struct {
        static bonType() {
            return MsgReadParam._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new MsgReadParam();
            o.conv_type = bb.readInt();
            o.convID = bb.readArray(() => {
                return bb.readInt();
            });
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.conv_type);
            bb.writeArray(this.convID, el => {
                bb.writeInt(el);
            });
            return bb;
        }
    }
    exports.MsgReadParam = MsgReadParam;
    MsgReadParam._$info = new sinfo_1.StructInfo("app_c/chat/server/chat.MsgReadParam", 2045486202, null, [new sinfo_1.FieldInfo("conv_type", new sinfo_1.EnumType(sinfo_1.Type.U8), null), new sinfo_1.FieldInfo("convID", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.U32)), null)]);
    struct_mgr_1.structMgr.register(MsgReadParam._$info.name_hash, MsgReadParam, MsgReadParam._$info.name);
    // 获取消息
    class GetMsgParam extends struct_mgr_1.Struct {
        static bonType() {
            return GetMsgParam._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new GetMsgParam();
            o.from = bb.readInt();
            o.conv_type = bb.readInt();
            o.count = bb.readInt();
            o.last_key = bb.readArray(() => {
                return bb.readInt();
            });
            o.msg_class = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.from);
            bb.writeInt(this.conv_type);
            bb.writeInt(this.count);
            bb.writeArray(this.last_key, el => {
                bb.writeInt(el);
            });
            bb.writeInt(this.msg_class);
            return bb;
        }
    }
    exports.GetMsgParam = GetMsgParam;
    GetMsgParam._$info = new sinfo_1.StructInfo("app_c/chat/server/chat.GetMsgParam", 1285047520, null, [new sinfo_1.FieldInfo("from", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("conv_type", new sinfo_1.EnumType(sinfo_1.Type.U8), null), new sinfo_1.FieldInfo("count", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("last_key", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.U32)), null), new sinfo_1.FieldInfo("msg_class", new sinfo_1.EnumType(sinfo_1.Type.U8), null)]);
    struct_mgr_1.structMgr.register(GetMsgParam._$info.name_hash, GetMsgParam, GetMsgParam._$info.name);
    // 消息游标
    class UserCursor extends struct_mgr_1.Struct {
        static bonType() {
            return UserCursor._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new UserCursor();
            o.rid = bb.readInt();
            o.cursor = bb.readInt();
            o.newId = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.rid);
            bb.writeInt(this.cursor);
            bb.writeInt(this.newId);
            return bb;
        }
    }
    exports.UserCursor = UserCursor;
    UserCursor._$info = new sinfo_1.StructInfo("app_c/chat/server/chat.UserCursor", 1747959986, null, [new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("cursor", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("newId", new sinfo_1.EnumType(sinfo_1.Type.U32), null)]);
    struct_mgr_1.structMgr.register(UserCursor._$info.name_hash, UserCursor, UserCursor._$info.name);
    // 通用返回值
    class RpcResult extends struct_mgr_1.Struct {
        static bonType() {
            return RpcResult._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new RpcResult();
            o.code = bb.readInt();
            if (!bb.isNil()) {
                o.value = bb.readUtf8();
            }
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.code);
            if (this.value === undefined || this.value === null) {
                bb.writeNil();
            } else {
                bb.writeUtf8(this.value);
            }
            return bb;
        }
    }
    exports.RpcResult = RpcResult;
    RpcResult._$info = new sinfo_1.StructInfo("app_c/chat/server/chat.RpcResult", 3535479428, null, [new sinfo_1.FieldInfo("code", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("value", new sinfo_1.EnumType(sinfo_1.Type.Option, new sinfo_1.EnumType(sinfo_1.Type.Str)), null)]);
    struct_mgr_1.structMgr.register(RpcResult._$info.name_hash, RpcResult, RpcResult._$info.name);
});