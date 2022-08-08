_$pi.define("app_a/struct/login.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo"], function (require, exports, module, struct_mgr_1, sinfo_1) {
    "use strict";

    exports.LoginResult = exports.ProofreadResult = void 0;
    class ProofreadResult extends struct_mgr_1.Struct {
        static bonType() {
            return ProofreadResult._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new ProofreadResult();
            o.clientTime = bb.readInt();
            o.serverTime = bb.readInt();
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.clientTime);
            bb.writeInt(this.serverTime);
            return bb;
        }
    }
    exports.ProofreadResult = ProofreadResult;
    ProofreadResult._$info = new sinfo_1.StructInfo("app_a/struct/login.ProofreadResult", 2929819687, null, [new sinfo_1.FieldInfo("clientTime", new sinfo_1.EnumType(sinfo_1.Type.Usize), null), new sinfo_1.FieldInfo("serverTime", new sinfo_1.EnumType(sinfo_1.Type.Usize), null)]);
    struct_mgr_1.structMgr.register(ProofreadResult._$info.name_hash, ProofreadResult, ProofreadResult._$info.name);
    // 登陆返回值
    class LoginResult extends struct_mgr_1.Struct {
        static bonType() {
            return LoginResult._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new LoginResult();
            o.code = bb.readInt();
            if (!bb.isNil()) {
                o.rid = bb.readInt();
            }
            if (!bb.isNil()) {
                o.seed = bb.readInt();
            }
            return o;
        }
        bonEncode(bb) {
            bb.writeInt(this.code);
            if (this.rid === undefined || this.rid === null) {
                bb.writeNil();
            } else {
                bb.writeInt(this.rid);
            }
            if (this.seed === undefined || this.seed === null) {
                bb.writeNil();
            } else {
                bb.writeInt(this.seed);
            }
            return bb;
        }
    }
    exports.LoginResult = LoginResult;
    LoginResult._$info = new sinfo_1.StructInfo("app_a/struct/login.LoginResult", 1096222913, null, [new sinfo_1.FieldInfo("code", new sinfo_1.EnumType(sinfo_1.Type.U32), null), new sinfo_1.FieldInfo("rid", new sinfo_1.EnumType(sinfo_1.Type.Option, new sinfo_1.EnumType(sinfo_1.Type.U32)), null), new sinfo_1.FieldInfo("seed", new sinfo_1.EnumType(sinfo_1.Type.Option, new sinfo_1.EnumType(sinfo_1.Type.U32)), null)]);
    struct_mgr_1.structMgr.register(LoginResult._$info.name_hash, LoginResult, LoginResult._$info.name);
});