module.exports = {
    extends: [
        "stylelint-config-standard",
        "stylelint-config-rational-order",
        "stylelint-config-recommended-less"
    ],
    overrides: [
        {
            files: ["**/*.less"],
            customSyntax: "postcss-less"
        },
        {
            files: ["**/*.scss"],
            customSyntax: "postcss-scss"
        }
    ],
    rules: {
        // 支持bem的规范
        'selector-class-pattern': '^([a-z][a-z0-9]*)(-+[a-z0-9]+)*$',
        // 支持element-plus的:deep伪类
        'selector-pseudo-class-no-unknown': [true, {
            ignorePseudoClasses: ["deep"]
        }],
        // at-rule-no-unknown: 屏蔽一些scss等语法检查
        "at-rule-no-unknown": [true, {
            "ignoreAtRules": [
                "tailwind"
            ]
        }],
    }
}
