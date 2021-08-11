func := $(lambda_name)

tgmusl = x86_64-unknown-linux-musl
rldir = target/$(tgmusl)/release
aws_ent = aws_entry
entrs = src/bin/$(aws_ent).rs
ent1 = $(rldir)/$(aws_ent)
outdir = dist
bsp := $(outdir)/bootstrap
azp := $(outdir)/app.zip
out1 := $(outdir)/out1.json
log1 := $(outdir)/log1
exm := event-example.json


$(ent1): $(entrs) src/*.rs
	cargo build --release --bin $(@F) --target $(tgmusl)

$(bsp): $(ent1)
	cp $< $@

$(azp): $(bsp)
	zip -j $@ $<

.PHONY: clean upload invoke log

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
