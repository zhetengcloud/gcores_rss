.PHONY: clean upload invoke log distdir aliyun aws

.SECONDARY:

dests = aliyun aws

aws_fn := $(aws_fn)
ali_fn := $(ali_fn)

tg_musl = x86_64-unknown-linux-musl
rl_dir = target/$(tg_musl)/release

dist = dist

exm := event-example.json

$(rl_dir)/%: src/**.rs
	cargo build --release --bin $(@F) --target $(tg_musl)

$(dist)/%/bootstrap: $(rl_dir)/%_entry
	cp $< $@

$(dist)/%/app.zip: $(dist)/%/bootstrap
	zip -j $@ $<

$(dests): %: $(dist)/%/app.zip

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
	rm -rf dist/*
