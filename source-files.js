var sourcesIndex = JSON.parse('{\
"anstream":["",[["adapter",[],["mod.rs","strip.rs","wincon.rs"]]],["auto.rs","buffer.rs","lib.rs","lockable.rs","macros.rs","raw.rs","strip.rs"]],\
"anstyle":["",[],["color.rs","effect.rs","lib.rs","macros.rs","reset.rs","style.rs"]],\
"anstyle_parse":["",[["state",[],["definitions.rs","mod.rs","table.rs"]]],["lib.rs","params.rs"]],\
"anstyle_query":["",[],["lib.rs","windows.rs"]],\
"arrayvec":["",[],["array_string.rs","arrayvec.rs","arrayvec_impl.rs","char.rs","errors.rs","lib.rs","utils.rs"]],\
"bitflags":["",[],["external.rs","internal.rs","iter.rs","lib.rs","parser.rs","public.rs","traits.rs"]],\
"byteorder":["",[],["io.rs","lib.rs"]],\
"clap":["",[],["lib.rs"]],\
"clap_builder":["",[["builder",[],["action.rs","app_settings.rs","arg.rs","arg_group.rs","arg_predicate.rs","arg_settings.rs","command.rs","debug_asserts.rs","ext.rs","mod.rs","os_str.rs","possible_value.rs","range.rs","resettable.rs","str.rs","styled_str.rs","styling.rs","value_hint.rs","value_parser.rs"]],["error",[],["context.rs","format.rs","kind.rs","mod.rs"]],["output",[["textwrap",[],["core.rs","mod.rs"]]],["fmt.rs","help.rs","help_template.rs","mod.rs","usage.rs"]],["parser",[["features",[],["mod.rs","suggestions.rs"]],["matches",[],["arg_matches.rs","matched_arg.rs","mod.rs","value_source.rs"]]],["arg_matcher.rs","error.rs","mod.rs","parser.rs","validator.rs"]],["util",[],["any_value.rs","color.rs","flat_map.rs","flat_set.rs","graph.rs","id.rs","mod.rs","str_to_bool.rs"]]],["derive.rs","lib.rs","macros.rs","mkeymap.rs"]],\
"clap_derive":["",[["derives",[],["args.rs","into_app.rs","mod.rs","parser.rs","subcommand.rs","value_enum.rs"]],["utils",[],["doc_comments.rs","error.rs","mod.rs","spanned.rs","ty.rs"]]],["attr.rs","dummies.rs","item.rs","lib.rs","macros.rs"]],\
"clap_lex":["",[],["ext.rs","lib.rs"]],\
"colorchoice":["",[],["lib.rs"]],\
"const_str":["",[["__ctfe",[],["ascii_case.rs","compare.rs","concat.rs","concat_bytes.rs","cstr.rs","encode.rs","eq_ignore_ascii_case.rs","equal.rs","find.rs","fmt.rs","hex.rs","is_ascii.rs","parse.rs","repeat.rs","replace.rs","sorted.rs","split.rs","squish.rs","str.rs","to_byte_array.rs","to_char_array.rs","to_str.rs","unwrap.rs"]]],["ascii.rs","bytes.rs","lib.rs","printable.rs","slice.rs","str.rs","utf16.rs","utf8.rs"]],\
"equivalent":["",[],["lib.rs"]],\
"fastrand":["",[],["lib.rs"]],\
"hashbrown":["",[["external_trait_impls",[],["mod.rs"]],["raw",[],["alloc.rs","bitmask.rs","mod.rs","sse2.rs"]]],["lib.rs","macros.rs","map.rs","scopeguard.rs","set.rs"]],\
"heck":["",[],["kebab.rs","lib.rs","lower_camel.rs","shouty_kebab.rs","shouty_snake.rs","snake.rs","title.rs","train.rs","upper_camel.rs"]],\
"indexmap":["",[["map",[["core",[],["raw.rs"]]],["core.rs","iter.rs","slice.rs"]],["set",[],["iter.rs","slice.rs"]]],["arbitrary.rs","lib.rs","macros.rs","map.rs","mutable_keys.rs","set.rs","util.rs"]],\
"is_terminal":["",[],["lib.rs"]],\
"itoa":["",[],["lib.rs","udiv128.rs"]],\
"linux_raw_sys":["",[["x86_64",[],["errno.rs","general.rs","ioctl.rs"]]],["lib.rs"]],\
"once_cell":["",[],["imp_std.rs","lib.rs","race.rs"]],\
"proc_macro2":["",[],["detection.rs","extra.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"rustix":["",[["backend",[["linux_raw",[["arch",[["asm",[],["mod.rs","x86_64.rs"]]],["mod.rs"]],["io",[],["errno.rs","mod.rs","syscalls.rs","types.rs"]],["termios",[],["mod.rs","syscalls.rs"]]],["c.rs","conv.rs","mod.rs","reg.rs"]]]],["io",[],["close.rs","dup.rs","errno.rs","fcntl.rs","ioctl.rs","mod.rs","read_write.rs"]],["maybe_polyfill",[["std",[],["mod.rs"]]]],["termios",[],["ioctl.rs","mod.rs","tc.rs","tty.rs","types.rs"]]],["bitcast.rs","cstr.rs","ffi.rs","lib.rs","pid.rs","utils.rs","weak.rs"]],\
"ryu":["",[["buffer",[],["mod.rs"]],["pretty",[],["exponent.rs","mantissa.rs","mod.rs"]]],["common.rs","d2s.rs","d2s_full_table.rs","d2s_intrinsics.rs","digit_table.rs","f2s.rs","f2s_intrinsics.rs","lib.rs"]],\
"serde":["",[["de",[],["format.rs","ignored_any.rs","impls.rs","mod.rs","seed.rs","size_hint.rs","utf8.rs","value.rs"]],["private",[],["de.rs","doc.rs","mod.rs","ser.rs"]],["ser",[],["fmt.rs","impls.rs","impossible.rs","mod.rs"]]],["integer128.rs","lib.rs","macros.rs"]],\
"serde_derive":["",[],["buffer.rs","bytecode.rs","lib.rs","lib_precompiled.rs"]],\
"serde_yaml":["",[["libyaml",[],["cstr.rs","emitter.rs","error.rs","mod.rs","parser.rs","tag.rs","util.rs"]],["value",[],["de.rs","debug.rs","from.rs","index.rs","mod.rs","partial_eq.rs","ser.rs","tagged.rs"]]],["de.rs","error.rs","lib.rs","loader.rs","mapping.rs","number.rs","path.rs","ser.rs","with.rs"]],\
"strsim":["",[],["lib.rs"]],\
"syn":["",[["gen",[],["clone.rs"]]],["attr.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","drops.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","gen_helper.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","meta.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","restriction.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]],\
"thiserror":["",[],["aserror.rs","display.rs","lib.rs"]],\
"thiserror_impl":["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","valid.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]],\
"unsafe_libyaml":["",[],["api.rs","dumper.rs","emitter.rs","lib.rs","loader.rs","macros.rs","parser.rs","reader.rs","scanner.rs","success.rs","writer.rs","yaml.rs"]],\
"utf8parse":["",[],["lib.rs","types.rs"]],\
"wasm4_stubs":["",[],["lib.rs"]],\
"wasm4_sx":["",[["music",[],["mod.rs"]],["tone",[],["adsr.rs","channel.rs","duty_cycle.rs","frequency_slide.rs","mod.rs","panning.rs","tone.rs","tone_flags.rs","volume.rs"]]],["cell.rs","color.rs","debug.rs","draw_colors.rs","engine.rs","gamepad.rs","lib.rs","mouse.rs","palette.rs","panic.rs","rand.rs","screen.rs","text.rs","vec2.rs"]],\
"wasm4_sys":["",[],["lib.rs"]],\
"wasm4_tracker":["",[],["args.rs","description.rs","lib.rs","tracker.rs"]]\
}');
createSourceSidebar();