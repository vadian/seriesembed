package emseries

import (
	"time"
)

type Criteria interface {
	apply(*Record) bool
}

type And struct {
	lside Criteria
	rside Criteria
}

type Or struct {
	lside Criteria
	rside Criteria
}

type Start struct {
	time      time.Time
	inclusive bool
}

type End struct {
	time      time.Time
	inclusive bool
}

type Tags struct {
	tags []string
}

/*
start time
end time
all tags
any tag
and / or
*/
