package emseries

import (
	"time"
)

type Criteria interface {
	apply(*Record) bool
}

type And struct {
	Lside Criteria
	Rside Criteria
}

func (self *And) apply(r *Record) bool {
	return false
}

type Or struct {
	lside Criteria
	rside Criteria
}

type Start struct {
	time      time.Time
	inclusive bool
}

func Start_(t time.Time, i bool) *Start {
	v := Start{t, i}
	return &v
}

func (self *Start) apply(r *Record) bool {
	return false
}

type End struct {
	time      time.Time
	inclusive bool
}

func End_(t time.Time, i bool) *End {
	v := End{t, i}
	return &v
}

func (self *End) apply(r *Record) bool {
	return false
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
