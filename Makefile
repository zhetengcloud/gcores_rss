.PHONY: clean upload_aws upload_ali invoke log_aws aliyun aws

.SECONDARY:

dests = aliyun aws

aws_fn := $(aws_fn)
ali_fn := $(ali_fn)
ali_service := $(ali_service)

tg_musl = x86_64-unknown-linux-musl
rl_dir = target/$(tg_musl)/release

dist = dist
aws_out := $(dist)/aws_out
aws_log := $(dist)/aws_log
exm := event-example.json

$(rl_dir)/%: src/*.rs src/bin/*.rs
	cargo build --release --bin $(@F) --target $(tg_musl)

$(dist)/%/bootstrap: $(rl_dir)/%_entry
	mkdir -p $(@D)
	cp $< $@

$(dist)/%/app.zip: $(dist)/%/bootstrap
	zip -j $@ $<

$(dests): %: $(dist)/%/app.zip

upload_aws: $(dist)/aws/app.zip
	aws lambda update-function-code --function-name $(aws_fn) --zip-file fileb://$<

invoke_aws:
	aws lambda invoke --function-name $(aws_fn) $(aws_out) \
	--output text --payload fileb://$(exm) \
	--log-type Tail > $(aws_log)

log_aws:
	grep -oE '\S{20,}' $(aws_log)| base64 -d
	cat $(aws_out)

upload_ali: $(dist)/aliyun/app.zip
	fcli function update --code-file $< -f $(ali_fn) -s $(ali_service)

invoke_ali:
	fcli function invoke -f $(ali_fn) -s $(ali_service) --event-file $(exm)

clean:
	cargo clean
	rm -rf dist/*
