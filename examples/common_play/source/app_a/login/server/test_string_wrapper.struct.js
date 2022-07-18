_$pi.define("app_a/login/server/test_string_wrapper.struct", ["require", "exports", "module", "pi_utils/serialization/struct_mgr", "pi_utils/serialization/sinfo", "pi_utils/serialization/string_wrapper"], function (require, exports, module, struct_mgr_1, sinfo_1, string_wrapper_1) {
    "use strict";

    exports.TestStringWrapper = void 0;
    class TestStringWrapper extends struct_mgr_1.Struct {
        static bonType() {
            return TestStringWrapper._$info.name_hash;
        }
        static bonDecode(bb) {
            let o;
            o = new TestStringWrapper();
            o.uid = bb.readUtf8();
            o.addr = o._$EnumTypeMap ? o._$EnumTypeMap(o.addr) : string_wrapper_1.StringWrapper.bonDecode(bb);
            o.data1 = bb.readArray(() => {
                return o._$EnumTypeMap ? o._$EnumTypeMap(o.data1) : string_wrapper_1.StringWrapper.bonDecode(bb);
            });
            o.data2 = bb.readMap(() => {
                return [bb.readUtf8(), o._$EnumTypeMap ? o._$EnumTypeMap(o.data2) : string_wrapper_1.StringWrapper.bonDecode(bb)];
            });
            return o;
        }
        bonEncode(bb) {
            bb.writeUtf8(this.uid);
            this.addr.bonEncode(bb);
            bb.writeArray(this.data1, el => {
                el.bonEncode(bb);
            });
            bb.writeMap(this.data2, (k, v) => {
                bb.writeUtf8(k);
                v.bonEncode(bb);
            });
            return bb;
        }
    }
    exports.TestStringWrapper = TestStringWrapper;
    TestStringWrapper._$info = new sinfo_1.StructInfo("app_a/login/server/test_string_wrapper.TestStringWrapper", 147316634, null, [new sinfo_1.FieldInfo("uid", new sinfo_1.EnumType(sinfo_1.Type.Str), null), new sinfo_1.FieldInfo("addr", new sinfo_1.EnumType(sinfo_1.Type.Struct, string_wrapper_1.StringWrapper._$info), null), new sinfo_1.FieldInfo("data1", new sinfo_1.EnumType(sinfo_1.Type.Arr, new sinfo_1.EnumType(sinfo_1.Type.Struct, string_wrapper_1.StringWrapper._$info)), null), new sinfo_1.FieldInfo("data2", new sinfo_1.EnumType(sinfo_1.Type.Map, [new sinfo_1.EnumType(sinfo_1.Type.Str), new sinfo_1.EnumType(sinfo_1.Type.Struct, string_wrapper_1.StringWrapper._$info)]), null)]);
    struct_mgr_1.structMgr.register(TestStringWrapper._$info.name_hash, TestStringWrapper, TestStringWrapper._$info.name);
});