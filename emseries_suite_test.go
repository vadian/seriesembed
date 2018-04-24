package emseries_test

import (
	. "github.com/onsi/ginkgo"
	. "github.com/onsi/gomega"

	"testing"
)

func TestEmseries(t *testing.T) {
	RegisterFailHandler(Fail)
	RunSpecs(t, "Emseries Suite")
}
