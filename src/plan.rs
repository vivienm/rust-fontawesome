/// Subscription plans.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Plan {
    /// The Free plan.
    Free,
    /// The Pro plan.
    Pro,
}

impl Plan {
    /// Returns `true` if the plan is [`Free`](Plan::Free).
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Plan;
    ///
    /// let plan = Plan::Free;
    /// assert_eq!(plan.is_free(), true);
    ///
    /// let plan = Plan::Pro;
    /// assert_eq!(plan.is_free(), false);
    /// ```
    #[inline]
    pub const fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }

    /// Returns `true` if the plan is [`Pro`](Plan::Pro).
    ///
    /// # Examples
    ///
    /// ```
    /// use fontawesome::Plan;
    ///
    /// let plan = Plan::Free;
    /// assert_eq!(plan.is_pro(), false);
    ///
    /// let plan = Plan::Pro;
    /// assert_eq!(plan.is_pro(), true);
    /// ```
    #[inline]
    pub const fn is_pro(&self) -> bool {
        matches!(self, Self::Pro)
    }
}
