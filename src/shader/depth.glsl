layout(set=1,binding=0)uniform Depth {
	// 深度值， 同一个节点在不同帧之间都有很大可能不同，需要经常更新，因此单独作为一个binding
	float depth;
};