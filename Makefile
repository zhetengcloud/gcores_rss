.PHONY: clean upload invoke log ali

.SECONDARY:

aws_fn := $(aws_fn)
ali_fn := $(ali_fn)

tg_musl = x86_64-unknown-linux-musl
rl_dir = target/$(tg_musl)/release

aws_ent = aws_entry
ali_ent = aliyun_entry

dist = dist

azp := $(dist)/app.zip
out1 := $(dist)/out1.json
log1 := $(dist)/log1
exm := event-example.json


$(rl_dir)/%: src/bin/**.rs
	cargo build --release --bin $(@F) --target $(tg_musl)

$(dist)/%: $(rl_dir)/%
	cp $< $@

ali_zip: $(dist)/$(ali_ent)



upload: $(azp)
	aws lambda update-function-code --function-name $(func) --zip-file fileb://$(azp)

invoke:
	aws lambda invoke --function-name $(func) $(out1) \
	--output text --payload fileb://$(exm) \
	--log-type Tail > $(log1)

log:
	grep -oE '\S{20,}' $(log1)| base64 -d
	cat $(out1)

clean:
	cargo clean
	rm -f dist/*
